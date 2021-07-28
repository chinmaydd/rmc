#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2106_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(5);
        buf.push_back(10);
        buf.push_back(15);
        assert_eq!(buf, [5, 10, 15]);

        buf.resize_with(5, Default::default);
        assert_eq!(buf, [5, 10, 15, 0, 0]);

        buf.resize_with(2, || unreachable!());
        assert_eq!(buf, [5, 10]);

        let mut state = 100;
        buf.resize_with(5, || {
            state += 1;
            state
        });
        assert_eq!(buf, [5, 10, 101, 102, 103]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2106_0()
}
