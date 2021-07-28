#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_136_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, String> = BTreeMap::new();
        let s = "hoho".to_string();

        map.entry("poneyland").or_insert_with(|| s);

        assert_eq!(map["poneyland"], "hoho".to_string());
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_136_0()
}
