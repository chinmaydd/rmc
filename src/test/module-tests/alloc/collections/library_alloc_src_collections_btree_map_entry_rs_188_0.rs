#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_188_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, usize> = BTreeMap::new();
        assert_eq!(map.entry("poneyland").key(), &"poneyland");
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_188_0()
}
