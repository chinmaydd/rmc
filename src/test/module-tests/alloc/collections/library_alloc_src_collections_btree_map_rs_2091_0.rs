#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_2091_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        a.insert(2, "b");
        a.insert(1, "a");

        let keys: Vec<_> = a.keys().cloned().collect();
        assert_eq!(keys, [1, 2]);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_2091_0()
}
