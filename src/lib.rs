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

#[cfg(test)]
mod tests {
    use crate::nf::*;
    use crate::nf_elem::*;
    use flint_sys::fmpz::*;
    use flint_sys::fmpq::*;
    use flint_sys::fmpq_poly::*;
    use std::mem::MaybeUninit;

    // Checks the norm of the element 2*x^2 - 1 of Q(x)/f where f = x^4 + 1
    #[test]
    fn try_it() {
        let mut a = MaybeUninit::uninit();
        let mut b = MaybeUninit::uninit();
        let mut res = MaybeUninit::uninit();
        let mut pol = MaybeUninit::uninit();

        let mut x = MaybeUninit::uninit();
        let mut nf = MaybeUninit::uninit();
        
        unsafe {
            fmpz_init_set_si(a.as_mut_ptr(), -1);
            let mut a = a.assume_init();
            fmpz_init_set_si(b.as_mut_ptr(), 2);
            let mut b = b.assume_init();

            fmpq_poly_init(pol.as_mut_ptr());
            let mut pol = pol.assume_init();

            fmpq_poly_set_coeff_ui(&mut pol, 0, 1);
            fmpq_poly_set_coeff_ui(&mut pol, 4, 1);

            nf_init(nf.as_mut_ptr(), &pol);
            let mut nf = nf.assume_init();

            nf_elem_init(x.as_mut_ptr(), &nf);
            let mut x = x.assume_init();
        
            _nf_elem_set_coeff_num_fmpz(&mut x, 0, &a, &nf);
            _nf_elem_set_coeff_num_fmpz(&mut x, 2, &b, &nf);

            fmpq_init(res.as_mut_ptr());
            let mut res = res.assume_init();
            nf_elem_norm(&mut res, &x, &nf);

            assert!(fmpq_equal_ui(&res, 25) != 0);
            println!("Success!");

            fmpz_clear(&mut a);
            fmpz_clear(&mut b);
            fmpq_clear(&mut res);
            fmpq_poly_clear(&mut pol);
            nf_elem_clear(&mut x, &nf);
            nf_clear(&mut nf);
        }
    }
}
