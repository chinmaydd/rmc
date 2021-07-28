#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_574_0() {
        use std::collections::VecDeque;

        let buf: VecDeque<i32> = VecDeque::with_capacity(10);
        assert!(buf.capacity() >= 10);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_574_0()
}
