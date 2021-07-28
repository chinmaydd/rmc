#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_518_0() {
        use std::collections::BTreeSet;

        let a: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
        let mut b = BTreeSet::new();

        assert_eq!(a.is_disjoint(&b), true);
        b.insert(4);
        assert_eq!(a.is_disjoint(&b), true);
        b.insert(1);
        assert_eq!(a.is_disjoint(&b), false);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_518_0()
}
