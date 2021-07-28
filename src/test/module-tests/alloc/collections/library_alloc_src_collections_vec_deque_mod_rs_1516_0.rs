#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1516_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        assert_eq!(buf.swap_remove_back(0), None);
        buf.push_back(1);
        buf.push_back(2);
        buf.push_back(3);
        assert_eq!(buf, [1, 2, 3]);

        assert_eq!(buf.swap_remove_back(0), Some(1));
        assert_eq!(buf, [3, 2]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1516_0()
}
