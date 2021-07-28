#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_1137_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        a.insert(1, "a");
        a.insert(2, "b");
        a.insert(3, "c");
        a.insert(17, "d");
        a.insert(41, "e");

        let b = a.split_off(&3);

        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 3);

        assert_eq!(a[&1], "a");
        assert_eq!(a[&2], "b");

        assert_eq!(b[&3], "c");
        assert_eq!(b[&17], "d");
        assert_eq!(b[&41], "e");
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_1137_0()
}
