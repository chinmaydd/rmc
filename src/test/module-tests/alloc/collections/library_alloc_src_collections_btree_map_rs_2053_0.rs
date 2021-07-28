#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_2053_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        // add 10 to the value if the key isn't "a"
        for (key, value) in map.iter_mut() {
            if key != &"a" {
                *value += 10;
            }
        }
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_2053_0()
}
