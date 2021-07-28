#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1255_0() {
        use std::collections::VecDeque;

        let mut vector: VecDeque<u32> = VecDeque::new();

        vector.push_back(0);
        vector.push_back(1);

        assert_eq!(vector.contains(&1), true);
        assert_eq!(vector.contains(&10), false);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1255_0()
}
