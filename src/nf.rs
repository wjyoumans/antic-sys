use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpq_poly::{fmpq_poly_powers_precomp_t, fmpq_poly_struct, fmpq_poly_t};
use flint_sys::fmpz::fmpz_preinvn_t;
use flint_sys::fmpz_poly::fmpz_poly_powers_precomp_t;

use libc::{c_long, c_ulong};

pub const NF_POWERS_CUTOFF: c_ulong = 30;
pub const NF_GENERIC: c_ulong = 0;
pub const NF_MONIC: c_ulong = 1;
pub const NF_LINEAR: c_ulong = 2;
pub const NF_QUADRATIC: c_ulong = 4;
pub const NF_GAUSSIAN: c_ulong = 8;

extern "C" {
    pub fn antic_test_multiplier() -> c_long;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_struct {
    pub pol: fmpq_poly_t,
    pub pinv: nf_struct__bindgen_ty_1,
    pub powers: nf_struct__bindgen_ty_2,
    pub traces: fmpq_poly_t,
    pub flag: mp_limb_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_struct__bindgen_ty_1 {
    pub qq: fmpz_preinvn_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_struct__bindgen_ty_2 {
    pub qq: fmpq_poly_powers_precomp_t,
    pub zz: fmpz_poly_powers_precomp_t,
}
pub type nf_t = [nf_struct; 1usize];
extern "C" {
    pub fn nf_init(nf: *mut nf_struct, pol: *const fmpq_poly_struct);
}
extern "C" {
    pub fn nf_init_randtest(
        nf: *mut nf_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits_in: mp_bitcnt_t,
    );
}
extern "C" {
    pub fn nf_clear(nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_print(nf: *const nf_struct);
}
