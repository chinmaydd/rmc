#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1280_0() {
        use std::collections::VecDeque;

        let mut d = VecDeque::new();
        assert_eq!(d.front(), None);

        d.push_back(1);
        d.push_back(2);
        assert_eq!(d.front(), Some(&1));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1280_0()
}
