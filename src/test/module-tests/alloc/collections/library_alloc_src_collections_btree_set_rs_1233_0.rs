#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_1233_0() {
        use std::collections::BTreeSet;

        let a: BTreeSet<_> = vec![1, 2, 3].into_iter().collect();
        let b: BTreeSet<_> = vec![2, 3, 4].into_iter().collect();

        let result = &a & &b;
        let result_vec: Vec<_> = result.into_iter().collect();
        assert_eq!(result_vec, [2, 3]);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_1233_0()
}
