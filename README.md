# Mumps-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

`Mumps-src` crate is a *-src crate. This links [Mumps] libraries to executable build by cargo, but does not provide Rust bindings. [Mumps] is built with [OpenBLAS] ([openblas-src])(Optional) or [Intel-MKL] ([intel-mkl-src])(Optional).

By this package, you don't need to worry about installing Mumps in the system, and it's a package for **all platforms(linux)**.

Mumps (MUltifrontal Massively Parallel sparse direct Solver) is is a package for solving systems of linear equations of the form Ax = b, where A is a square sparse matrix that can be either unsymmetric, symmetric positive definite, or general symmetric, on distributed memory computers.

**Note:** Only sequential version is supported now. And `Metis` is not supported now.

## Usage

1. Add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    mumps-src = "\*"
    ```

2. Add the following to your `lib.rs`:

    ```toml
    extern crate mumps_src;
    ```

This package does not provide bindings.

## Configure

### Features

The following Cargo features are supported:

* `default` to `openblas-static` feature and `d_arith` feature;

At least 1 following feature need to be selected:

* `openblas-system` to link to OpenBLAS system library;
* `openblas-static` to link to OpenBLAS statically;
* `intel-mkl-system` to link to intel-mkl system library;
* `intel-mkl-static` to link to intel-mkl statically;

There's 4 kinds of precision for mumps, and at least 1 following feature need to be selected:

* `s_arith` single, real, precision;
* `d_arith` double, real, precision;
* `c_arith` single, complex, precision;
* `z_arith` double, complex, precision;

### Environment

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_MUMPS_STATIC` to link to Mumps statically;
* `CARGO_MUMPS_SYSTEM` to link to Mumps system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

If you enable OpenBLAS([openblas-src]), you can also pass env to `make` by `OPENBLAS_*`. Read more at [here](#cross-compilation)

### Others

If you enable OpenBLAS([openblas-src]), you can link `OpenBLAS` staticaly or dynamicly by disable default feature and select what you like, for example:

```toml
mumps-src = { version = "\*", default-features = no, features = ["d_arith", "openblas-system"] }
```

Similarly, you can link Intel MKL([intel-mkl-src]) with:

```toml
mumps-src = { version = "\*", default-features = no, features = ["d_arith", "intel-mkl-system"] }
```

If you want more configuration, you can try this:

```toml
mumps-src = { version = "\*", default-features = no, features = ["d_arith"] }
intel-mkl-src = { version = "\*", features = ["mkl-static-lp64-seq"] }
```

## Cross Compilation

If you use OpenBLAS([openblas-src]), you need to set `OPENBLAS_CC`, `OPENBLAS_FC`, `OPENBLAS_HOSTCC`, and `OPENBLAS_TARGET` to pass env to [OpenBLAS], ref:[openblas-src] and [OpenBLAS]. For example:

```sh
export OPENBLAS_TARGET=ARMV8
export OPENBLAS_HOSTCC=gcc
export OPENBLAS_CC=aarch64-linux-gnu-gcc
export OPENBLAS_FC=aarch64-linux-gnu-gfortran
```

You can compile it for the other target by providing the `--target` option to `cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `aarch64-unknown-linux-gnu`          | ✓   |
| `aarch64-unknown-linux-musl`         | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-ps-windows-msvc`             | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |
| `x86_64-unknown-linux-musl`          | ✓   |
| others                               | not test   |

Note: Features `intel-mkl-*` can only be used for `x86_64-*`. Features `openblas-static` can only be used for `linux`.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[Mumps]: https://mumps-solver.org/
[OpenBLAS]: https://github.com/OpenMathLib/OpenBLAS
[intel-mkl]: https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html

[openblas-src]: https://github.com/blas-lapack-rs/openblas-src
[intel-mkl-src]: https://github.com/rust-math/intel-mkl-src

[documentation-img]: https://docs.rs/mumps-src/badge.svg
[documentation-url]: https://docs.rs/mumps-src
[package-img]: https://img.shields.io/crates/v/mumps-src.svg
[package-url]: https://crates.io/crates/mumps-src
[license-img]: https://img.shields.io/crates/l/mumps-src.svg
[license-url]: https://github.com/Maroon502/mumps-src/blob/master/LICENSE.md
