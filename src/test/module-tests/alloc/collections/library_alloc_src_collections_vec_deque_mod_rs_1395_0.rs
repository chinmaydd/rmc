#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1395_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        assert_eq!(buf.pop_back(), None);
        buf.push_back(1);
        buf.push_back(3);
        assert_eq!(buf.pop_back(), Some(3));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1395_0()
}
