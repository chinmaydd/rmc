#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_259_0() {
        let mut v = [-5, 4, 1, -3, 2];

        v.sort();
        assert!(v == [-5, -3, 1, 2, 4]);
    }
    _doctest_main_library_alloc_src_slice_rs_259_0()
}
