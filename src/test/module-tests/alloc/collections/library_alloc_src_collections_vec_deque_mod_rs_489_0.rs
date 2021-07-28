#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_489_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(3);
        buf.push_back(4);
        buf.push_back(5);
        assert_eq!(buf.get(1), Some(&4));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_489_0()
}
