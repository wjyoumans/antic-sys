//! Bindings for the [Antic](https://github.com/wbhart/antic.git) library.

#![allow(non_camel_case_types)]

use libc::c_ulong;

pub const NF_POWERS_CUTOFF: c_ulong = 30;
pub const NF_GENERIC: c_ulong = 0;
pub const NF_MONIC: c_ulong = 1;
pub const NF_LINEAR: c_ulong = 2;
pub const NF_QUADRATIC: c_ulong = 4;
pub const NF_GAUSSIAN: c_ulong = 8;

pub mod nf;
pub mod nf_elem;
pub mod qfb;
