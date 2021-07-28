#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_804_0() {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();

        set.insert(2);
        assert_eq!(set.remove(&2), true);
        assert_eq!(set.remove(&2), false);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_804_0()
}
