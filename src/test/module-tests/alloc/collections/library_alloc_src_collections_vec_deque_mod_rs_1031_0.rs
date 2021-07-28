#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1031_0() {
        use std::collections::VecDeque;

        let mut v = VecDeque::new();
        assert_eq!(v.len(), 0);
        v.push_back(1);
        assert_eq!(v.len(), 1);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1031_0()
}
