#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1701_0() {
        let mut vec = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];
        vec.append(&mut vec2);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
        assert_eq!(vec2, []);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1701_0()
}
