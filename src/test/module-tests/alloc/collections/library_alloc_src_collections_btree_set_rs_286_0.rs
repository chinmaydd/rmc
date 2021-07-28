#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_286_0() {
        use std::collections::BTreeSet;

        let mut a = BTreeSet::new();
        a.insert(1);
        a.insert(2);

        let mut b = BTreeSet::new();
        b.insert(2);
        b.insert(3);

        let diff: Vec<_> = a.difference(&b).cloned().collect();
        assert_eq!(diff, [1]);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_286_0()
}
