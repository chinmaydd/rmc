#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_1035_0() {
        use std::collections::BTreeSet;

        let mut v = BTreeSet::new();
        assert!(v.is_empty());
        v.insert(1);
        assert!(!v.is_empty());
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_1035_0()
}
