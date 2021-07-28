#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_622_0() {
        use std::collections::VecDeque;

        let mut buf: VecDeque<i32> = vec![1].into_iter().collect();
        buf.reserve(10);
        assert!(buf.capacity() >= 11);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_622_0()
}
