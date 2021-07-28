#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1165_0() {
        use std::collections::VecDeque;

        let mut v: VecDeque<_> = vec![1, 2, 3].into_iter().collect();
        let drained = v.drain(2..).collect::<VecDeque<_>>();
        assert_eq!(drained, [3]);
        assert_eq!(v, [1, 2]);

        // A full range clears all contents
        v.drain(..);
        assert!(v.is_empty());
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1165_0()
}
