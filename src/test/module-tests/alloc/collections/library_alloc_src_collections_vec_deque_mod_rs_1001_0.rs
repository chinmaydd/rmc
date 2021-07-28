#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1001_0() {
        use std::collections::VecDeque;

        let mut vector = VecDeque::new();

        vector.push_back(0);
        vector.push_back(1);

        vector.push_front(10);
        vector.push_front(9);

        vector.as_mut_slices().0[0] = 42;
        vector.as_mut_slices().1[0] = 24;
        assert_eq!(vector.as_slices(), (&[42, 10][..], &[24, 1][..]));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1001_0()
}
