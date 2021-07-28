#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_281_0() {
        use std::collections::btree_map::Entry;
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, usize> = BTreeMap::new();

        if let Entry::Vacant(v) = map.entry("poneyland") {
            v.into_key();
        }
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_281_0()
}
