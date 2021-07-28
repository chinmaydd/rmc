#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1419_0() {
        use std::collections::VecDeque;

        let mut d = VecDeque::new();
        d.push_front(1);
        d.push_front(2);
        assert_eq!(d.front(), Some(&2));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1419_0()
}
