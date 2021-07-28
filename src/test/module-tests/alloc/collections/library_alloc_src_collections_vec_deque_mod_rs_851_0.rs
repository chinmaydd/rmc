#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_851_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(5);
        buf.push_back(10);
        buf.push_back(15);
        assert_eq!(buf, [5, 10, 15]);
        buf.truncate(1);
        assert_eq!(buf, [5]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_851_0()
}
