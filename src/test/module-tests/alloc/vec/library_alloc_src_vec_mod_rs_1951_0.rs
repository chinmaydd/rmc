#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1951_0() {
        let x = vec![1, 2, 3];
        let static_ref: &'static mut [usize] = x.leak();
        static_ref[0] += 1;
        assert_eq!(static_ref, &[2, 2, 3]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1951_0()
}
