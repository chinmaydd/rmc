#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2303_0() {
        use std::collections::VecDeque;

        let mut buf: VecDeque<_> = (0..10).collect();

        buf.rotate_left(3);
        assert_eq!(buf, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        for i in 1..10 {
            assert_eq!(i * 3 % 10, buf[0]);
            buf.rotate_left(3);
        }
        assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2303_0()
}
