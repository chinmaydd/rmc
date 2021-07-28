#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_301_0() {
        use std::collections::btree_map::Entry;
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, u32> = BTreeMap::new();

        if let Entry::Vacant(o) = map.entry("poneyland") {
            o.insert(37);
        }
        assert_eq!(map["poneyland"], 37);
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_301_0()
}
