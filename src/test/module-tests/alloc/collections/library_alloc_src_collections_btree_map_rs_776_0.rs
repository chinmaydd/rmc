#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_776_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(1, "a");
        if let Some(x) = map.get_mut(&1) {
            *x = "b";
        }
        assert_eq!(map[&1], "b");
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_776_0()
}
