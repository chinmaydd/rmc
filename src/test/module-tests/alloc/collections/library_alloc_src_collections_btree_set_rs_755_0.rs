#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_755_0() {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();

        assert_eq!(set.insert(2), true);
        assert_eq!(set.insert(2), false);
        assert_eq!(set.len(), 1);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_755_0()
}
