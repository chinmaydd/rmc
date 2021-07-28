#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1850_0() {
        let mut vec = vec![1, 2, 3];
        let vec2 = vec.split_off(1);
        assert_eq!(vec, [1]);
        assert_eq!(vec2, [2, 3]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1850_0()
}
