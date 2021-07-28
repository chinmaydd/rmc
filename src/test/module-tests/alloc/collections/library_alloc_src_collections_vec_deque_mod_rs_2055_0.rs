#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2055_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.extend(1..6);

        let keep = [false, true, true, false, true];
        let mut i = 0;
        buf.retain(|_| (keep[i], i += 1).0);
        assert_eq!(buf, [2, 3, 5]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2055_0()
}
