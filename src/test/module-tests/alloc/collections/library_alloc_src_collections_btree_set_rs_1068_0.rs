#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_1068_0() {
        use std::collections::BTreeSet;

        let set: BTreeSet<usize> = [1, 2, 3, 4].iter().cloned().collect();

        let v: Vec<_> = set.into_iter().collect();
        assert_eq!(v, [1, 2, 3, 4]);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_1068_0()
}
