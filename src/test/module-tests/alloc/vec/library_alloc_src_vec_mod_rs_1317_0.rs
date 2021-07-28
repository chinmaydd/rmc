#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1317_0() {
        let mut vec = vec![1, 2, 3];
        vec.insert(1, 4);
        assert_eq!(vec, [1, 4, 2, 3]);
        vec.insert(4, 5);
        assert_eq!(vec, [1, 4, 2, 3, 5]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1317_0()
}
