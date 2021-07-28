#![allow(unused_variables)]
#![deny(warnings)]
#![feature(map_first_last)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_581_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        assert_eq!(map.first_key_value(), None);
        map.insert(1, "b");
        map.insert(2, "a");
        assert_eq!(map.first_key_value(), Some((&1, &"b")));
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_581_0()
}
