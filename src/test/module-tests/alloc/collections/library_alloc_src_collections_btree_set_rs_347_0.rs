#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_347_0() {
        use std::collections::BTreeSet;

        let mut a = BTreeSet::new();
        a.insert(1);
        a.insert(2);

        let mut b = BTreeSet::new();
        b.insert(2);
        b.insert(3);

        let sym_diff: Vec<_> = a.symmetric_difference(&b).cloned().collect();
        assert_eq!(sym_diff, [1, 3]);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_347_0()
}
