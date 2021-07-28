#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1080_0() {
        use std::collections::VecDeque;

        let v: VecDeque<_> = vec![1, 2, 3].into_iter().collect();
        let range = v.range(2..).copied().collect::<VecDeque<_>>();
        assert_eq!(range, [3]);

        // A full range covers all contents
        let all = v.range(..);
        assert_eq!(all.len(), 3);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1080_0()
}
