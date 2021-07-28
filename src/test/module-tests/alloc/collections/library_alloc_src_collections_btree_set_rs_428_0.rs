#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_428_0() {
        use std::collections::BTreeSet;

        let mut a = BTreeSet::new();
        a.insert(1);

        let mut b = BTreeSet::new();
        b.insert(2);

        let union: Vec<_> = a.union(&b).cloned().collect();
        assert_eq!(union, [1, 2]);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_428_0()
}
