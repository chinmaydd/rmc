#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_750_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(1, "a");
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_750_0()
}
