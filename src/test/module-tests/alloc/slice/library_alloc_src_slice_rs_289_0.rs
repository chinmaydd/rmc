#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_289_0() {
        let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
        floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
    }
    _doctest_main_library_alloc_src_slice_rs_289_0()
}
