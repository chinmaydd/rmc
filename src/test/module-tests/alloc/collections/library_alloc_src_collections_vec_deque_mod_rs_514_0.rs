#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_514_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(3);
        buf.push_back(4);
        buf.push_back(5);
        if let Some(elem) = buf.get_mut(1) {
            *elem = 7;
        }

        assert_eq!(buf[1], 7);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_514_0()
}
