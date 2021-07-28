#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_244_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, Option<usize>> = BTreeMap::new();
        map.entry("poneyland").or_default();

        assert_eq!(map["poneyland"], None);
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_244_0()
}
