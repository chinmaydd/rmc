#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1957_0() {
        use std::collections::VecDeque;

        let mut buf: VecDeque<_> = vec![1, 2, 3].into_iter().collect();
        let buf2 = buf.split_off(1);
        assert_eq!(buf, [1]);
        assert_eq!(buf2, [2, 3]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1957_0()
}
