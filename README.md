# Mumps-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

Mumps-src crate is a *-src crate. This links [Mumps] libraries to executable build by cargo, but does not provide Rust bindings.

By this package, you don't need to worry about installing Mumps in the system, and it's a package for **all platforms(linux)**.

Mumps (MUltifrontal Massively Parallel sparse direct Solver) is is a package for solving systems of linear equations of the form Ax = b, where A is a square sparse matrix that can be either unsymmetric, symmetric positive definite, or general symmetric, on distributed memory computers.

**attention:** Only sequential version is supported now. And `Metis` is not supported now.

## Usage

1. Add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    mumps-src = "0.1"
    ```

2. Add the following to your `lib.rs`:

    ```toml
    extern crate mumps_src;
    ```

This package does not provide bindings.

## Configure

The following Cargo features are supported:

* `default` to `openblas-static` feature and `d_arith` feature;
* `openblas-system` to link to Openblas system library;
* `openblas-static` to link to Openblas statically;
* `intel-mkl-system` to link to intel-mkl system library;
* `intel-mkl-static` to link to intel-mkl statically;

There's 4 kinds of precision for mumps:

* `s_arith` single, real, precision;
* `d_arith` double, real, precision;
* `c_arith` single, complex, precision;
* `z_arith` double, complex, precision;

## Cross Compilation

Because [openblas-src]'s Issue [#101](https://github.com/blas-lapack-rs/openblas-src/issues/101), we can't cross compile the package with `openblas-static` feature. So, if you want to cross compile the package, you could use [mike-kfed](https://github.com/mike-kfed/openblas-src/tree/arm-cross-compile) instead.

Add this to your `project/.cargo/config.toml`.

```toml
[patch.crates-io]
openblas-src = { git = "https://github.com/mike-kfed/openblas-src.git", branch = "arm-cross-compile" }
```

you can compile it for the other target by providing the `--target` option to `cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-linux-androideabi`            | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[Mumps]: https://mumps-solver.org/

[documentation-img]: https://docs.rs/mumps-src/badge.svg
[documentation-url]: https://docs.rs/mumps-src
[package-img]: https://img.shields.io/crates/v/mumps-src.svg
[package-url]: https://crates.io/crates/mumps-src
[license-img]: https://img.shields.io/crates/l/mumps-src.svg
[license-url]: https://github.com/Maroon502/mumps-src/blob/master/LICENSE.md
