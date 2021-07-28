#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1368_0() {
        use std::collections::VecDeque;

        let mut d = VecDeque::new();
        d.push_back(1);
        d.push_back(2);

        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_front(), Some(2));
        assert_eq!(d.pop_front(), None);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1368_0()
}
