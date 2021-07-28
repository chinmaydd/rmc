#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_2021_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(3, "c");
        map.insert(2, "b");
        map.insert(1, "a");

        for (key, value) in map.iter() {
            println!("{}: {}", key, value);
        }

        let (first_key, first_value) = map.iter().next().unwrap();
        assert_eq!((*first_key, *first_value), (1, "a"));
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_2021_0()
}
