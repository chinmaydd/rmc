#![allow(unused_variables)]
#![deny(warnings)]
#![feature(map_first_last)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_724_0() {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();

        set.insert(1);
        while let Some(n) = set.pop_last() {
            assert_eq!(n, 1);
        }
        assert!(set.is_empty());
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_724_0()
}
