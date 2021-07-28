#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_1265_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        a.insert(1, "hello");
        a.insert(2, "goodbye");

        let values: Vec<&str> = a.into_values().collect();
        assert_eq!(values, ["hello", "goodbye"]);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_1265_0()
}
