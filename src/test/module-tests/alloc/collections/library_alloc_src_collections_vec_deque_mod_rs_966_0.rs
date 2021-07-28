#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_966_0() {
        use std::collections::VecDeque;

        let mut vector = VecDeque::new();

        vector.push_back(0);
        vector.push_back(1);
        vector.push_back(2);

        assert_eq!(vector.as_slices(), (&[0, 1, 2][..], &[][..]));

        vector.push_front(10);
        vector.push_front(9);

        assert_eq!(vector.as_slices(), (&[9, 10][..], &[0, 1, 2][..]));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_966_0()
}
