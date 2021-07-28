#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2883_0() {
        use std::collections::VecDeque;

        // This one is *O*(1).
        let deque: VecDeque<_> = (1..5).collect();
        let ptr = deque.as_slices().0.as_ptr();
        let vec = Vec::from(deque);
        assert_eq!(vec, [1, 2, 3, 4]);
        assert_eq!(vec.as_ptr(), ptr);

        // This one needs data rearranging.
        let mut deque: VecDeque<_> = (1..5).collect();
        deque.push_front(9);
        deque.push_front(8);
        let ptr = deque.as_slices().1.as_ptr();
        let vec = Vec::from(deque);
        assert_eq!(vec, [8, 9, 1, 2, 3, 4]);
        assert_eq!(vec.as_ptr(), ptr);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2883_0()
}
