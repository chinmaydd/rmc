#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_452_0() {
        use std::collections::BTreeSet;

        let mut v = BTreeSet::new();
        v.insert(1);
        v.clear();
        assert!(v.is_empty());
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_452_0()
}
