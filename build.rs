use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use cc::Build;
use coin_build_tools::{coinbuilder,link, utils};

const LIB_NAME: &str = "Mumps";
const LIB_VERSION: &str = "5.7.2";

fn main() {
    let download_dir = if let Ok(p) = env::var("MUMPS_DOWNLOAD_DIR") {
        p
    } else {
        env::var("OUT_DIR").unwrap()
    };

    let want_system = utils::want_system(LIB_NAME);

    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        return;
    }

    let lib_dir = format!(
        "{}/{}_{}",
        download_dir,
        LIB_NAME.to_uppercase(),
        LIB_VERSION
    );
    if !Path::new(&format!("{}/LICENSE", lib_dir)).exists() {
        download_mumps(download_dir, LIB_VERSION).unwrap();
    }
    build_lib_and_link(lib_dir.as_str());
}

fn build_lib_and_link(lib_dir: &str) {
    let _target = env::var("TARGET").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(&out_dir).join("build");

    //config
    let mut config = coinbuilder::init_builder();
    config.cpp(false).out_dir(target_dir.clone());

    config.define("Add_", None); //Add_, Add__ for G95,  UPPER

    config
        .flag(&format!("-J{}", target_dir.display()))
        .flag("-fallow-argument-mismatch");

    let src_dir = PathBuf::from(lib_dir).join("src");
    let mut includes_dir = vec![format!("{}/include", lib_dir)];

    let path = target_dir.join("mumps_int_def.h");
    std::fs::create_dir_all(&target_dir).expect("cannot create target_dir");
    let mut file = std::fs::File::create(path).unwrap();
    let buf = b"#if ! defined(MUMPS_INT_H)
#define MUMPS_INT_H
#define MUMPS_INTSIZE32 //default to int32
// #define MUMPS_INTSIZE64
#endif
    ";
    file.write_all(buf).expect("Write Error!");
    file.flush().expect("Write Error!");
    includes_dir.push(target_dir.display().to_string());

    // build common
    let (files_common_mod, files_common_oth) = Mumps_files_common();
    let files_common_mod = files_common_mod
        .iter()
        .map(|f| format!("{}/{}", src_dir.display(), f))
        .collect::<Vec<_>>();
    let mut files_common_oth = files_common_oth
        .iter()
        .map(|f| format!("{}/{}", src_dir.display(), f))
        .collect::<Vec<_>>();
    // libseq
    includes_dir.push(format!("{}/libseq", lib_dir));
    files_common_oth.push(format!("{}/libseq/mpi.f", lib_dir));
    files_common_oth.push(format!("{}/libseq/mpic.c", lib_dir));
    files_common_oth.push(format!("{}/libseq/elapse.c", lib_dir));

    //include
    config.includes(includes_dir.clone());

    // compile
    for (i, file_common_mod) in files_common_mod.iter().enumerate() {
        config
            .clone()
            .file(file_common_mod)
            .compile(&format!("Mumps_common{}", i));
    }

    config
        .clone()
        .files(files_common_oth)
        .compile("Mumps_common");

    // build sdcz
    #[cfg(feature = "s_arith")]
    build_mumps_arith(config.clone(), "s", src_dir);
    #[cfg(feature = "d_arith")]
    build_mumps_arith(config.clone(), "d", src_dir);
    #[cfg(feature = "c_arith")]
    build_mumps_arith(config.clone(), "c", src_dir);
    #[cfg(feature = "z_arith")]
    build_mumps_arith(config.clone(), "z", src_dir);

    coinbuilder::print_metadata(includes_dir, vec![]);
}

fn build_mumps_arith(mut config: Build, arith: &str, src_dir: PathBuf) {
    config.define(
        "MUMPS_ARITH",
        Some(format!("MUMPS_ARITH_{}", arith).as_str()),
    );

    let (files_arith_mod, files_arith_oth) = Mumps_files_arith(arith);
    let files_arith_mod = files_arith_mod
        .iter()
        .map(|f| format!("{}/{}", src_dir.display(), f))
        .collect::<Vec<_>>();
    let files_arith_oth = files_arith_oth
        .iter()
        .map(|f| format!("{}/{}", src_dir.display(), f))
        .collect::<Vec<_>>();

    // compile
    for (i, file_arith_mod) in files_arith_mod.iter().enumerate() {
        config
            .clone()
            .file(file_arith_mod)
            .compile(&format!("Mumps_arith_{}{}", arith, i));
    }

    config
        .clone()
        .files(files_arith_oth)
        .compile(&format!("Mumps_arith_{}", arith));
}

pub fn download_mumps<P: AsRef<Path>>(download_dir: P, lib_version: &str) -> Result<PathBuf> {
    let lib_name = LIB_NAME;
    let url = format!("http://mumps-solver.org/MUMPS_{}.tar.gz", lib_version);
    let dest = download_dir
        .as_ref()
        .join(format!("{}_{}", lib_name.to_uppercase(), lib_version));
    if !dest.exists() {
        let buf = get_agent().get(url.as_str()).call()?.into_reader();
        let gz_stream = flate2::read::GzDecoder::new(buf);
        let mut ar = tar::Archive::new(gz_stream);
        ar.unpack(&download_dir)?;
        assert!(dest.exists());
    }
    Ok(dest)
}

fn get_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .tls_connector(std::sync::Arc::new(
            native_tls::TlsConnector::new().expect("failed to create TLS connector"),
        ))
        .build()
}

#[allow(non_snake_case)]
fn Mumps_files_common() -> (Vec<String>, Vec<String>) {
    let files_common_mod = vec![
        "ana_blk_m.F".to_string(),
        "double_linked_list.F".to_string(),
        "ana_omp_m.F".to_string(),
        "ana_orderings_wrappers_m.F".to_string(),
        "omp_tps_common_m.F".to_string(),
        "fac_asm_build_sort_index_ELT_m.F".to_string(),
        "fac_asm_build_sort_index_m.F".to_string(),
        "front_data_mgt_m.F".to_string(),
        "fac_descband_data_m.F".to_string(),
        "fac_future_niv2_mod.F".to_string(),
        "fac_ibct_data_m.F".to_string(),
        "fac_maprow_data_m.F".to_string(),
        "lr_common.F".to_string(),
        "mumps_l0_omp_m.F".to_string(),
        "mumps_memory_mod.F".to_string(),
        "mumps_mpitoomp_m.F".to_string(),
        "mumps_ooc_common.F".to_string(),
        "mumps_static_mapping.F".to_string(),
        "mumps_pivnul_mod.F".to_string(),
    ];
    let files_common_oth = vec![
        "sol_ds_common_m.F".to_string(),
        "mumps_comm_ibcast.F".to_string(),
        "ana_AMDMF.F".to_string(),
        "ana_blk.F".to_string(),
        "ana_orderings.F".to_string(),
        "ana_set_ordering.F".to_string(),
        "bcast_errors.F".to_string(),
        "estim_flops.F".to_string(),
        "mumps_print_defined.F".to_string(),
        "mumps_type2_blocking.F".to_string(),
        "mumps_version.F".to_string(),
        "sol_common.F".to_string(),
        "tools_common.F".to_string(),
        "mumps_addr.c".to_string(),
        // "mumps_c.c".to_string(),
        "mumps_common.c".to_string(),
        "mumps_config_file_C.c".to_string(),
        "mumps_io_basic.c".to_string(),
        "mumps_io.c".to_string(),
        "mumps_io_err.c".to_string(),
        "mumps_io_thread.c".to_string(),
        "mumps_metis64.c".to_string(),
        "mumps_metis.c".to_string(),
        "mumps_metis_int.c".to_string(),
        "mumps_numa.c".to_string(),
        "mumps_pord.c".to_string(),
        "mumps_save_restore_C.c".to_string(),
        "mumps_scotch64.c".to_string(),
        "mumps_scotch.c".to_string(),
        "mumps_scotch_int.c".to_string(),
        "mumps_thread_affinity.c".to_string(),
        "mumps_register_thread.c".to_string(),
        "mumps_thread.c".to_string(),
    ];

    (files_common_mod, files_common_oth)
}

#[allow(non_snake_case)]
fn Mumps_files_arith(arith: &str) -> (Vec<String>, Vec<String>) {
    // files start with d,s,z,c,  d for double, s for single
    let files_mod = vec![
        "mumps_struc_def.F".to_string(),
        "ana_aux.F".to_string(),
        "ana_aux_par.F".to_string(),
        "lr_type.F".to_string(),
        "mumps_lr_data_m.F".to_string(),
        "lr_stats.F".to_string(),
        "lr_core.F".to_string(),
        "mumps_comm_buffer.F".to_string(),
        "mumps_load.F".to_string(),
        "ana_lr.F".to_string(),
        "fac_sol_l0omp_m.F".to_string(),
        "mumps_ooc_buffer.F".to_string(),
        "mumps_ooc.F".to_string(),
        "static_ptr_m.F".to_string(),
        "fac_mem_dynamic.F".to_string(),
        "omp_tps_m.F".to_string(),
        "fac_lr.F".to_string(),
        "fac_front_aux.F".to_string(),
        "fac_front_type2_aux.F".to_string(),
        "fac_front_LDLT_type1.F".to_string(),
        "fac_front_LDLT_type2.F".to_string(),
        "fac_front_LU_type1.F".to_string(),
        "fac_front_LU_type2.F".to_string(),
        "fac_asm_master_ELT_m.F".to_string(),
        "fac_asm_master_m.F".to_string(),
        "fac_sispointers_m.F".to_string(),
        "fac_omp_m.F".to_string(),
        "fac_par_m.F".to_string(),
        "mumps_mpi3_mod.F".to_string(),
        "mumps_save_restore_files.F".to_string(),
        "mumps_save_restore.F".to_string(),
        "mumps_sol_es.F".to_string(),
        "sol_lr.F".to_string(),
        "sol_omp_m.F".to_string(),
        "mumps_mpi3_mod.F".to_string(),
    ];

    let files_oth = vec![
        "ana_dist_m.F".to_string(),
        "ana_aux_ELT.F".to_string(),
        "ana_driver.F".to_string(),
        "ana_LDLT_preprocess.F".to_string(),
        "ana_mtrans.F".to_string(),
        "ana_reordertree.F".to_string(),
        "arrowheads.F".to_string(),
        "bcast_int.F".to_string(),
        "end_driver.F".to_string(),
        "fac_asm_ELT.F".to_string(),
        "fac_asm.F".to_string(),
        "fac_b.F".to_string(),
        "fac_determinant.F".to_string(),
        "fac_diag.F".to_string(),
        "fac_dist_arrowheads_omp.F".to_string(),
        "fac_distrib_distentry.F".to_string(),
        "fac_distrib_ELT.F".to_string(),
        "fac_driver.F".to_string(),
        "fac_lastrtnelind.F".to_string(),
        "fac_mem_alloc_cb.F".to_string(),
        "fac_mem_compress_cb.F".to_string(),
        "fac_mem_free_block_cb.F".to_string(),
        "fac_mem_stack_aux.F".to_string(),
        "fac_mem_stack.F".to_string(),
        "fac_process_band.F".to_string(),
        "fac_process_bf.F".to_string(),
        "fac_process_blfac_slave.F".to_string(),
        "fac_process_blocfacto.F".to_string(),
        "fac_process_blocfacto_LDLT.F".to_string(),
        "fac_process_contrib_type1.F".to_string(),
        "fac_process_contrib_type2.F".to_string(),
        "fac_process_contrib_type3.F".to_string(),
        "fac_process_end_facto_slave.F".to_string(),
        "fac_process_maprow.F".to_string(),
        "fac_process_master2.F".to_string(),
        "fac_process_message.F".to_string(),
        "fac_process_root2slave.F".to_string(),
        "fac_process_root2son.F".to_string(),
        "fac_process_rtnelind.F".to_string(),
        "fac_root_parallel.F".to_string(),
        "fac_scalings.F".to_string(),
        "fac_scalings_simScaleAbs.F".to_string(),
        "fac_scalings_simScale_util.F".to_string(),
        "fac_sol_pool.F".to_string(),
        "fac_type3_symmetrize.F".to_string(),
        "ini_defaults.F".to_string(),
        "ini_driver.F".to_string(),
        "mumps_config_file.F".to_string(),
        "mumps_driver.F".to_string(),
        "mumps_f77.F".to_string(),
        "mumps_iXamax.F".to_string(),
        "ooc_panel_piv.F".to_string(),
        "rank_revealing.F".to_string(),
        "sol_aux.F".to_string(),
        "sol_bwd_aux.F".to_string(),
        "sol_bwd.F".to_string(),
        "sol_c.F".to_string(),
        "sol_distrhs.F".to_string(),
        "sol_driver.F".to_string(),
        "sol_fwd_aux.F".to_string(),
        "sol_fwd.F".to_string(),
        "sol_matvec.F".to_string(),
        "sol_root_parallel.F".to_string(),
        "tools.F".to_string(),
        "type3_root.F".to_string(),
        "mumps_gpu.c".to_string(),
    ];

    let files_arith_mod = files_mod
        .iter()
        .map(|f| format!("{}{}", arith, f))
        .collect();
    let mut files_arith_oth: Vec<_> = files_oth
        .iter()
        .map(|f| format!("{}{}", arith, f))
        .collect();
    files_arith_oth.push("mumps_c.c".to_string());

    (files_arith_mod, files_arith_oth)
}
