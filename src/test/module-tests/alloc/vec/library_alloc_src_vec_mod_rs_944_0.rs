#![allow(unused_variables)]
#![deny(warnings)]
#![feature(shrink_to)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_944_0() {
        let mut vec = Vec::with_capacity(10);
        vec.extend([1, 2, 3]);
        assert_eq!(vec.capacity(), 10);
        vec.shrink_to(4);
        assert!(vec.capacity() >= 4);
        vec.shrink_to(0);
        assert!(vec.capacity() >= 3);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_944_0()
}
