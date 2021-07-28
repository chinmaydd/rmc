#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_460_0() {
        use std::collections::btree_map::Entry;
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, usize> = BTreeMap::new();
        map.entry("poneyland").or_insert(12);

        if let Entry::Occupied(mut o) = map.entry("poneyland") {
            assert_eq!(o.insert(15), 12);
        }
        assert_eq!(map["poneyland"], 15);
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_460_0()
}
