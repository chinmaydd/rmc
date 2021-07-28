#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_2159_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        assert_eq!(a.len(), 0);
        a.insert(1, "a");
        assert_eq!(a.len(), 1);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_2159_0()
}
