#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_entry_rs_207_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, usize> = BTreeMap::new();

        map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map["poneyland"], 42);

        map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map["poneyland"], 43);
    }
    _doctest_main_library_alloc_src_collections_btree_map_entry_rs_207_0()
}
