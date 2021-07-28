#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_543_0() {
        use std::collections::BTreeSet;

        let sup: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
        let mut set = BTreeSet::new();

        assert_eq!(set.is_subset(&sup), true);
        set.insert(2);
        assert_eq!(set.is_subset(&sup), true);
        set.insert(4);
        assert_eq!(set.is_subset(&sup), false);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_543_0()
}
