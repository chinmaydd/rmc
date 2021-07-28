#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2262_0() {
        let mut vec = vec![1, 2, 2, 3, 2];

        vec.dedup();

        assert_eq!(vec, [1, 2, 3, 2]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2262_0()
}
