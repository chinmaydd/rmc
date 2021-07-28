#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_549_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(3);
        buf.push_back(4);
        buf.push_back(5);
        assert_eq!(buf, [3, 4, 5]);
        buf.swap(0, 2);
        assert_eq!(buf, [5, 4, 3]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_549_0()
}
