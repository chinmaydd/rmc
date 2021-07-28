#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_2133_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        a.insert(1, String::from("hello"));
        a.insert(2, String::from("goodbye"));

        for value in a.values_mut() {
            value.push_str("!");
        }

        let values: Vec<String> = a.values().cloned().collect();
        assert_eq!(values, [String::from("hello!"), String::from("goodbye!")]);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_2133_0()
}
