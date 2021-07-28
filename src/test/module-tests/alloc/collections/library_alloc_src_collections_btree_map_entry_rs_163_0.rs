#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_163_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, usize> = BTreeMap::new();

        map.entry("poneyland")
            .or_insert_with_key(|key| key.chars().count());

        assert_eq!(map["poneyland"], 9);
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_163_0()
}
