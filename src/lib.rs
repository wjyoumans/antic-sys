//! Bindings for the [Antic](https://github.com/wbhart/antic.git) library.
//! Crates marked with an asterisk have functions which may require mutable borrows where const
//! borrows will suffice (these need to be corrected but the bindings will still work as expected).
#![allow(non_camel_case_types)]

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpz::{fmpz, fmpz_t, fmpz_preinvn_t};
use flint_sys::fmpq::fmpq;
use flint_sys::fmpz_mod::fmpz_mod_ctx_struct;
use flint_sys::fmpz_poly::fmpz_poly_powers_precomp_t;
use flint_sys::fmpq_poly::{fmpq_poly_struct, fmpq_poly_t, fmpq_poly_powers_precomp_t};
use flint_sys::fmpz_mod_poly::fmpz_mod_poly_struct;
use flint_sys::nmod_poly::nmod_poly_struct;
use flint_sys::fmpz_mat::fmpz_mat_struct;
use flint_sys::fmpq_mat::fmpq_mat_struct;

extern "C" {
    pub fn antic_test_multiplier() -> ::std::os::raw::c_long;
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
    pub fn nf_init(nf: *mut nf_struct, pol: *mut fmpq_poly_struct);
}
extern "C" {
    pub fn nf_init_randtest(
        nf: *mut nf_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits_in: mp_bitcnt_t,
    );
}
extern "C" {
    pub fn nf_clear(nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_print(nf: *mut nf_struct);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lnf_elem_struct {
    pub num: fmpz_t,
    pub den: fmpz_t,
}
pub type lnf_elem_t = [lnf_elem_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qnf_elem_struct {
    pub num: [fmpz; 3usize],
    pub den: fmpz_t,
}
pub type qnf_elem_t = [qnf_elem_struct; 1usize];
#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_elem_struct {
    pub elem: fmpq_poly_t,
    pub lelem: lnf_elem_t,
    pub qelem: qnf_elem_t,
}
pub type nf_elem_t = [nf_elem_struct; 1usize];
extern "C" {
    pub fn nf_elem_init(a: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_elem_clear(a: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_elem_randtest(
        a: *mut nf_elem_struct,
        state: *mut flint_rand_s,
        bits: mp_bitcnt_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_randtest_not_zero(
        a: *mut nf_elem_struct,
        state: *mut flint_rand_s,
        bits: mp_bitcnt_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_reduce(a: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_elem_reduce(a: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn _nf_elem_invertible_check(
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _nf_elem_equal(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        nf: *mut nf_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nf_elem_equal(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        nf: *mut nf_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nf_elem_is_gen(a: *mut nf_elem_struct, nf: *mut nf_struct) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nf_elem_print_pretty(
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
        var: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn nf_elem_get_str_pretty(
        a: *mut nf_elem_struct,
        var: *const ::std::os::raw::c_char,
        nf: *mut nf_struct,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn nf_elem_set_fmpq_poly(
        a: *mut nf_elem_struct,
        pol: *mut fmpq_poly_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_set_fmpz_mat_row(
        b: *mut nf_elem_struct,
        M: *mut fmpz_mat_struct,
        i: mp_limb_signed_t,
        den: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_get_fmpz_mat_row(
        M: *mut fmpz_mat_struct,
        i: mp_limb_signed_t,
        den: *mut fmpz,
        b: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_get_fmpq_poly(
        pol: *mut fmpq_poly_struct,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_get_nmod_poly(
        pol: *mut nmod_poly_struct,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_get_nmod_poly_den(
        pol: *mut nmod_poly_struct,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
        den: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn nf_elem_get_nmod_poly(
        pol: *mut nmod_poly_struct,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_get_fmpz_mod_poly(
        pol: *mut fmpz_mod_poly_struct,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn nf_elem_get_fmpz_mod_poly_den(
        pol: *mut fmpz_mod_poly_struct,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
        den: ::std::os::raw::c_int,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn nf_elem_get_fmpz_mod_poly(
        pol: *mut fmpz_mod_poly_struct,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn nf_elem_get_coeff_fmpq(
        a: *mut fmpq,
        b: *mut nf_elem_struct,
        i: mp_limb_signed_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_get_coeff_fmpz(
        a: *mut fmpz,
        b: *mut nf_elem_struct,
        i: mp_limb_signed_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_set_coeff_num_fmpz(
        a: *mut nf_elem_struct,
        i: mp_limb_signed_t,
        b: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_add_si(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_add_fmpz(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_add_fmpq(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpq,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_sub_si(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_sub_fmpz(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_sub_fmpq(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpq,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_si_sub(
        a: *mut nf_elem_struct,
        c: mp_limb_signed_t,
        b: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_fmpz_sub(
        a: *mut nf_elem_struct,
        c: *mut fmpz,
        b: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_fmpq_sub(
        a: *mut nf_elem_struct,
        c: *mut fmpq,
        b: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_scalar_mul_si(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_scalar_mul_fmpz(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_scalar_mul_fmpq(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpq,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_scalar_div_si(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_scalar_div_fmpz(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_scalar_div_fmpq(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut fmpq,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_add_lf(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
        can: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _nf_elem_sub_lf(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
        can: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _nf_elem_add_qf(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
        can: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _nf_elem_sub_qf(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
        can: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn nf_elem_add_qf(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_sub_qf(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_mul_gen(a: *mut nf_elem_struct, b: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn _nf_elem_mul(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_mul(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_mul_red(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
        red: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn nf_elem_mul_red(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
        red: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _nf_elem_inv(a: *mut nf_elem_struct, b: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_elem_inv(a: *mut nf_elem_struct, b: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn _nf_elem_div(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_div(
        a: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        c: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_pow(
        res: *mut nf_elem_struct,
        b: *mut nf_elem_struct,
        e: mp_limb_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_pow(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        e: mp_limb_t,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_norm(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_norm(res: *mut fmpq, a: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn _nf_elem_norm_div(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
        divisor: *mut fmpz,
        nbits: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn nf_elem_norm_div(
        res: *mut fmpq,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
        divisor: *mut fmpz,
        nbits: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _nf_elem_trace(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_trace(res: *mut fmpq, a: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_elem_rep_mat(res: *mut fmpq_mat_struct, a: *mut nf_elem_struct, nf: *mut nf_struct);
}
extern "C" {
    pub fn nf_elem_rep_mat_fmpz_mat_den(
        res: *mut fmpz_mat_struct,
        den: *mut fmpz,
        a: *mut nf_elem_struct,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn _nf_elem_mod_fmpz(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        mod_: *mut fmpz,
        nf: *mut nf_struct,
        sign: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn nf_elem_mod_fmpz_den(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        mod_: *mut fmpz,
        nf: *mut nf_struct,
        den: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn nf_elem_smod_fmpz_den(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        mod_: *mut fmpz,
        nf: *mut nf_struct,
        den: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn nf_elem_mod_fmpz(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        mod_: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_smod_fmpz(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        mod_: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_coprime_den(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        mod_: *mut fmpz,
        nf: *mut nf_struct,
    );
}
extern "C" {
    pub fn nf_elem_coprime_den_signed(
        res: *mut nf_elem_struct,
        a: *mut nf_elem_struct,
        mod_: *mut fmpz,
        nf: *mut nf_struct,
    );
}
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
    pub fn qfb_hash_init(depth: mp_limb_signed_t) -> *mut qfb_hash_t;
}
extern "C" {
    pub fn qfb_hash_clear(qhash: *mut qfb_hash_t, depth: mp_limb_signed_t);
}
extern "C" {
    pub fn qfb_hash_insert(
        qhash: *mut qfb_hash_t,
        q: *mut qfb,
        q2: *mut qfb,
        iter: mp_limb_signed_t,
        depth: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn qfb_hash_find(
        qhash: *mut qfb_hash_t,
        q: *mut qfb,
        depth: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn qfb_reduce(r: *mut qfb, f: *mut qfb, D: *mut fmpz);
}
extern "C" {
    pub fn qfb_is_reduced(r: *mut qfb) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfb_reduced_forms(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
}
extern "C" {
    pub fn qfb_reduced_forms_large(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
}
extern "C" {
    pub fn qfb_nucomp(r: *mut qfb, f: *mut qfb, g: *mut qfb, D: *mut fmpz, L: *mut fmpz);
}
extern "C" {
    pub fn qfb_nudupl(r: *mut qfb, f: *mut qfb, D: *mut fmpz, L: *mut fmpz);
}
extern "C" {
    pub fn qfb_pow_ui(r: *mut qfb, f: *mut qfb, D: *mut fmpz, exp: mp_limb_t);
}
extern "C" {
    pub fn qfb_pow(r: *mut qfb, f: *mut qfb, D: *mut fmpz, exp: *mut fmpz);
}
extern "C" {
    pub fn qfb_pow_with_root(r: *mut qfb, f: *mut qfb, D: *mut fmpz, e: *mut fmpz, L: *mut fmpz);
}
extern "C" {
    pub fn qfb_prime_form(r: *mut qfb, D: *mut fmpz, p: *mut fmpz);
}
extern "C" {
    pub fn qfb_exponent_element(
        exponent: *mut fmpz,
        f: *mut qfb,
        n: *mut fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfb_exponent(
        exponent: *mut fmpz,
        n: *mut fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
        c: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfb_exponent_grh(
        exponent: *mut fmpz,
        n: *mut fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> ::std::os::raw::c_int;
}

