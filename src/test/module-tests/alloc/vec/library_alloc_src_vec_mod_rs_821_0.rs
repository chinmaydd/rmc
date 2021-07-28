#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_821_0() {
        let mut vec = vec![1];
        vec.reserve_exact(10);
        assert!(vec.capacity() >= 11);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_821_0()
}
