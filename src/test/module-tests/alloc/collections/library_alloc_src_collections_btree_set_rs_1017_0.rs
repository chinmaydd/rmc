#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_1017_0() {
        use std::collections::BTreeSet;

        let mut v = BTreeSet::new();
        assert_eq!(v.len(), 0);
        v.insert(1);
        assert_eq!(v.len(), 1);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_1017_0()
}
