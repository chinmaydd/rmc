#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2020_0() {
        use std::collections::VecDeque;

        let mut buf: VecDeque<_> = vec![1, 2].into_iter().collect();
        let mut buf2: VecDeque<_> = vec![3, 4].into_iter().collect();
        buf.append(&mut buf2);
        assert_eq!(buf, [1, 2, 3, 4]);
        assert_eq!(buf2, []);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2020_0()
}
