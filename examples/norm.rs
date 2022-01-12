use std::mem::MaybeUninit;

use antic_sys::nf::*;
use antic_sys::nf_elem::*;
use flint_sys::fmpz::*;
use flint_sys::fmpq::*;
use flint_sys::fmpq_poly::*;

fn main() {
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
