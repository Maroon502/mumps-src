#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Ensure the libraries are linked in, despite it not being used directly
#[cfg(any(feature = "intel-mkl-static", feature = "intel-mlk-system"))]
extern crate intel_mkl_src;
#[cfg(any(feature = "openblas-static", feature = "openblas-system"))]
extern crate openblas_src;
