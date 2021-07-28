#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_503_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        a.insert(1, "a");
        a.clear();
        assert!(a.is_empty());
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_503_0()
}
