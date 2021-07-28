#![allow(unused_variables)]
#![deny(warnings)]
#![feature(shrink_to)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_768_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::with_capacity(15);
        buf.extend(0..4);
        assert_eq!(buf.capacity(), 15);
        buf.shrink_to(6);
        assert!(buf.capacity() >= 6);
        buf.shrink_to(0);
        assert!(buf.capacity() >= 4);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_768_0()
}
