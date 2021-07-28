#![allow(unused_variables)]
#![deny(warnings)]
#![feature(map_first_last)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_688_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        if let Some(mut entry) = map.last_entry() {
            if *entry.key() > 0 {
                entry.insert("last");
            }
        }
        assert_eq!(*map.get(&1).unwrap(), "a");
        assert_eq!(*map.get(&2).unwrap(), "last");
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_688_0()
}
