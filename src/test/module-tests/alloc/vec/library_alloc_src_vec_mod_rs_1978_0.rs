#![allow(unused_variables)]
#![deny(warnings)]
#![feature(vec_spare_capacity, maybe_uninit_extra)]

// Allocate vector big enough for 10 elements.
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1978_0() {
        let mut v = Vec::with_capacity(10);

        // Fill in the first 3 elements.
        let uninit = v.spare_capacity_mut();
        uninit[0].write(0);
        uninit[1].write(1);
        uninit[2].write(2);

        // Mark the first 3 elements of the vector as being initialized.
        unsafe {
            v.set_len(3);
        }

        assert_eq!(&v, &[0, 1, 2]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1978_0()
}
