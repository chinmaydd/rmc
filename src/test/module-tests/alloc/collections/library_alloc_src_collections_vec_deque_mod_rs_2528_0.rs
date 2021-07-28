#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2528_0() {
        use std::collections::VecDeque;

        let deque: VecDeque<_> = vec![
            (0, 0),
            (2, 1),
            (4, 1),
            (5, 1),
            (3, 1),
            (1, 2),
            (2, 3),
            (4, 5),
            (5, 8),
            (3, 13),
            (1, 21),
            (2, 34),
            (4, 55),
        ]
        .into();

        assert_eq!(deque.binary_search_by_key(&13, |&(a, b)| b), Ok(9));
        assert_eq!(deque.binary_search_by_key(&4, |&(a, b)| b), Err(7));
        assert_eq!(deque.binary_search_by_key(&100, |&(a, b)| b), Err(13));
        let r = deque.binary_search_by_key(&1, |&(a, b)| b);
        assert!(matches!(r, Ok(1..=4)));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2528_0()
}
