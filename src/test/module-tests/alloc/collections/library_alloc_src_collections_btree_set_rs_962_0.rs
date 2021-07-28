#![allow(unused_variables)]
#![deny(warnings)]
#![feature(btree_drain_filter)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_962_0() {
        use std::collections::BTreeSet;

        let mut set: BTreeSet<i32> = (0..8).collect();
        let evens: BTreeSet<_> = set.drain_filter(|v| v % 2 == 0).collect();
        let odds = set;
        assert_eq!(evens.into_iter().collect::<Vec<_>>(), vec![0, 2, 4, 6]);
        assert_eq!(odds.into_iter().collect::<Vec<_>>(), vec![1, 3, 5, 7]);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_962_0()
}
