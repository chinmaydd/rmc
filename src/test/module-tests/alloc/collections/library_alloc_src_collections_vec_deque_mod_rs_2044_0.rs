#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2044_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.extend(1..5);
        buf.retain(|&x| x % 2 == 0);
        assert_eq!(buf, [2, 4]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2044_0()
}
