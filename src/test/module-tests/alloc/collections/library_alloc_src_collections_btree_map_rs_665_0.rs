#![allow(unused_variables)]
#![deny(warnings)]
#![feature(map_first_last)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_665_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(1, "b");
        map.insert(2, "a");
        assert_eq!(map.last_key_value(), Some((&2, &"a")));
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_665_0()
}
