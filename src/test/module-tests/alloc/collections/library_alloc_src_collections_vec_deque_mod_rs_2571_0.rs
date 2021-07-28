#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2571_0() {
        use std::collections::VecDeque;

        let deque: VecDeque<_> = vec![1, 2, 3, 3, 5, 6, 7].into();
        let i = deque.partition_point(|&x| x < 5);

        assert_eq!(i, 4);
        assert!(deque.iter().take(i).all(|&x| x < 5));
        assert!(deque.iter().skip(i).all(|&x| !(x < 5)));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2571_0()
}
