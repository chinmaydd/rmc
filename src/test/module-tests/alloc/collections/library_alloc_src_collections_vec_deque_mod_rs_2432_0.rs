#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2432_0() {
        use std::collections::VecDeque;

        let mut deque: VecDeque<_> = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
        let num = 42;
        let idx = deque.binary_search(&num).unwrap_or_else(|x| x);
        deque.insert(idx, num);
        assert_eq!(deque, &[0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2432_0()
}
