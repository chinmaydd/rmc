#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_553_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(1, "a");
        assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
        assert_eq!(map.get_key_value(&2), None);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_553_0()
}
