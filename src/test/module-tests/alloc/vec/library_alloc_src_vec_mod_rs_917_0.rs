#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_917_0() {
        let mut vec = Vec::with_capacity(10);
        vec.extend([1, 2, 3]);
        assert_eq!(vec.capacity(), 10);
        vec.shrink_to_fit();
        assert!(vec.capacity() >= 3);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_917_0()
}
