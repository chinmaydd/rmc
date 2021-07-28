#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1675_0() {
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec, [1, 2]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1675_0()
}
