#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1420_0() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let keep = [false, true, true, false, true];
        let mut iter = keep.iter();
        vec.retain(|_| *iter.next().unwrap());
        assert_eq!(vec, [2, 3, 5]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1420_0()
}
