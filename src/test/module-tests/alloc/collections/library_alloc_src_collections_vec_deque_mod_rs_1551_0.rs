#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1551_0() {
        use std::collections::VecDeque;

        let mut vec_deque = VecDeque::new();
        vec_deque.push_back('a');
        vec_deque.push_back('b');
        vec_deque.push_back('c');
        assert_eq!(vec_deque, &['a', 'b', 'c']);

        vec_deque.insert(1, 'd');
        assert_eq!(vec_deque, &['a', 'd', 'b', 'c']);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1551_0()
}
