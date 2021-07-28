#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_1025_0() {
        use std::collections::BTreeMap;
        use std::ops::Bound::Included;

        let mut map = BTreeMap::new();
        map.insert(3, "a");
        map.insert(5, "b");
        map.insert(8, "c");
        for (&key, &value) in map.range((Included(&4), Included(&8))) {
            println!("{}: {}", key, value);
        }
        assert_eq!(Some((&5, &"b")), map.range(4..).next());
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_1025_0()
}
