#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_311_0() {
        let mut v = [5, 4, 1, 3, 2];
        v.sort_by(|a, b| a.cmp(b));
        assert!(v == [1, 2, 3, 4, 5]);

        // reverse sorting
        v.sort_by(|a, b| b.cmp(a));
        assert!(v == [5, 4, 3, 2, 1]);
    }
    _doctest_main_library_alloc_src_slice_rs_311_0()
}
