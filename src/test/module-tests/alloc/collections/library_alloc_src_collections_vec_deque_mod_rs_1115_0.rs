#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1115_0() {
        use std::collections::VecDeque;

        let mut v: VecDeque<_> = vec![1, 2, 3].into_iter().collect();
        for v in v.range_mut(2..) {
            *v *= 2;
        }
        assert_eq!(v, vec![1, 2, 6]);

        // A full range covers all contents
        for v in v.range_mut(..) {
            *v *= 2;
        }
        assert_eq!(v, vec![2, 4, 12]);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1115_0()
}
