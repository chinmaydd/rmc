#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_189_0() {
        let vec = vec![0; 5];
        assert_eq!(vec, [0, 0, 0, 0, 0]);

        // The following is equivalent, but potentially slower:
        let mut vec = Vec::with_capacity(5);
        vec.resize(5, 0);
        assert_eq!(vec, [0, 0, 0, 0, 0]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_189_0()
}
