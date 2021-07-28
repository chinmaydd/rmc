#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_2179_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        assert!(a.is_empty());
        a.insert(1, "a");
        assert!(!a.is_empty());
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_2179_0()
}
