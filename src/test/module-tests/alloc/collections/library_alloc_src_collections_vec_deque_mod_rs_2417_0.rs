#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2417_0() {
        use std::collections::VecDeque;

        let deque: VecDeque<_> = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();

        assert_eq!(deque.binary_search(&13), Ok(9));
        assert_eq!(deque.binary_search(&4), Err(7));
        assert_eq!(deque.binary_search(&100), Err(13));
        let r = deque.binary_search(&1);
        assert!(matches!(r, Ok(1..=4)));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2417_0()
}
