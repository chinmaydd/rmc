#![allow(unused_variables)]
#![deny(warnings)]
#![feature(map_first_last)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_649_0() {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();
        assert_eq!(set.first(), None);
        set.insert(1);
        assert_eq!(set.first(), Some(&1));
        set.insert(2);
        assert_eq!(set.first(), Some(&1));
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_649_0()
}
