#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_355_0() {
        let mut v = [-5i32, 4, 1, -3, 2];

        v.sort_by_key(|k| k.abs());
        assert!(v == [1, 2, -3, 4, -5]);
    }
    _doctest_main_library_alloc_src_slice_rs_355_0()
}
