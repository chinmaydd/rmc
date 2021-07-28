#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1236_0() {
        use std::collections::VecDeque;

        let mut v = VecDeque::new();
        v.push_back(1);
        v.clear();
        assert!(v.is_empty());
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1236_0()
}
