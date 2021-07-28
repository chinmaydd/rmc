#![allow(unused_variables)]
#![deny(warnings)]
#![feature(map_try_insert)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_850_0() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        assert_eq!(map.try_insert(37, "a").unwrap(), &"a");

        let err = map.try_insert(37, "b").unwrap_err();
        assert_eq!(err.entry.key(), &37);
        assert_eq!(err.entry.get(), &"a");
        assert_eq!(err.value, "b");
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_850_0()
}
