#![allow(unused_variables)]
#![deny(warnings)]
#![feature(map_first_last)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_721_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        while let Some((key, _val)) = map.pop_last() {
            assert!(map.iter().all(|(k, _v)| *k < key));
        }
        assert!(map.is_empty());
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_721_0()
}
