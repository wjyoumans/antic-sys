use flint_sys::deps::*;
use flint_sys::fmpz::{fmpz, fmpz_t};

use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qfb {
    pub a: fmpz_t,
    pub b: fmpz_t,
    pub c: fmpz_t,
}
pub type qfb_t = [qfb; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qfb_hash_t {
    pub q: qfb_t,
    pub q2: qfb_t,
    pub iter: mp_limb_signed_t,
}
extern "C" {
    pub fn qfb_hash_init(depth: mp_limb_signed_t) -> *const qfb_hash_t;
}
extern "C" {
    pub fn qfb_hash_clear(qhash: *mut qfb_hash_t, depth: mp_limb_signed_t);
}
extern "C" {
    pub fn qfb_hash_insert(
        qhash: *mut qfb_hash_t,
        q: *const qfb,
        q2: *const qfb,
        iter: mp_limb_signed_t,
        depth: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn qfb_hash_find(
        qhash: *mut qfb_hash_t,
        q: *const qfb,
        depth: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn qfb_reduce(r: *mut qfb, f: *const qfb, D: *const fmpz);
}
extern "C" {
    pub fn qfb_is_reduced(r: *const qfb) -> c_int;
}
extern "C" {
    pub fn qfb_reduced_forms(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
}
extern "C" {
    pub fn qfb_reduced_forms_large(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
}
extern "C" {
    pub fn qfb_nucomp(r: *mut qfb, f: *const qfb, g: *const qfb, D: *const fmpz, L: *const fmpz);
}
extern "C" {
    pub fn qfb_nudupl(r: *mut qfb, f: *const qfb, D: *const fmpz, L: *const fmpz);
}
extern "C" {
    pub fn qfb_pow_ui(r: *mut qfb, f: *const qfb, D: *const fmpz, exp: mp_limb_t);
}
extern "C" {
    pub fn qfb_pow(r: *mut qfb, f: *const qfb, D: *const fmpz, exp: *const fmpz);
}
extern "C" {
    pub fn qfb_pow_with_root(
        r: *mut qfb,
        f: *const qfb,
        D: *const fmpz,
        e: *const fmpz,
        L: *const fmpz,
    );
}
extern "C" {
    pub fn qfb_prime_form(r: *mut qfb, D: *const fmpz, p: *const fmpz);
}
extern "C" {
    pub fn qfb_exponent_element(
        exponent: *mut fmpz,
        f: *const qfb,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn qfb_exponent(
        exponent: *mut fmpz,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
        c: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn qfb_exponent_grh(
        exponent: *mut fmpz,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> c_int;
}
