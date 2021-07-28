#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_396_0() {
        let mut v = [-5i32, 4, 32, -3, 2];

        v.sort_by_cached_key(|k| k.to_string());
        assert!(v == [-3, -5, 2, 32, 4]);
    }
    _doctest_main_library_alloc_src_slice_rs_396_0()
}
