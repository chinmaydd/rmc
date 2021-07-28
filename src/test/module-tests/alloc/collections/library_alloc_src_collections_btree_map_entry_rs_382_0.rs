#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_382_0() {
        use std::collections::btree_map::Entry;
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, usize> = BTreeMap::new();
        map.entry("poneyland").or_insert(12);

        if let Entry::Occupied(o) = map.entry("poneyland") {
            assert_eq!(o.get(), &12);
        }
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_382_0()
}
