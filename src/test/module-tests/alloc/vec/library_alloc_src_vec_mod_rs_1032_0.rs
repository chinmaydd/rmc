#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1032_0() {
        let mut vec = vec![1, 2, 3];
        vec.truncate(0);
        assert_eq!(vec, []);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1032_0()
}
