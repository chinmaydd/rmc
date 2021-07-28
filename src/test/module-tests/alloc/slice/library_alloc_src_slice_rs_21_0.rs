#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_21_0() {
        let x = &mut [1, 2, 3];
        x[1] = 7;
        assert_eq!(x, &[1, 7, 3]);
    }
    _doctest_main_library_alloc_src_slice_rs_21_0()
}
