#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1773_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(1);
        buf.push_back(2);
        buf.push_back(3);
        assert_eq!(buf, [1, 2, 3]);

        assert_eq!(buf.remove(1), Some(2));
        assert_eq!(buf, [1, 3]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1773_0()
}
