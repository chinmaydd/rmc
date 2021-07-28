#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1444_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(1);
        buf.push_back(3);
        assert_eq!(3, *buf.back().unwrap());
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1444_0()
}
