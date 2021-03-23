#![no_std]
#![allow(incomplete_features)]
#![feature(repr_simd, platform_intrinsics, simd_ffi, const_generics)]
#![warn(missing_docs)]
//! Portable SIMD module.

#[macro_use]
mod first;
#[macro_use]
mod permute;
#[macro_use]
mod transmute;

mod comparisons;
mod fmt;
mod intrinsics;
mod ops;
mod round;

mod lanes_at_most_64;
pub use lanes_at_most_64::LanesAtMost64;

mod masks;
pub use masks::*;

mod vector;
pub use vector::*;
