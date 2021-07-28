#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2140_0() {
        let mut vec = vec![1];
        vec.extend_from_slice(&[2, 3, 4]);
        assert_eq!(vec, [1, 2, 3, 4]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2140_0()
}
