#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_963_0() {
        use std::collections::BTreeMap;

        let mut a = BTreeMap::new();
        a.insert(1, "a");
        a.insert(2, "b");
        a.insert(3, "c");

        let mut b = BTreeMap::new();
        b.insert(3, "d");
        b.insert(4, "e");
        b.insert(5, "f");

        a.append(&mut b);

        assert_eq!(a.len(), 5);
        assert_eq!(b.len(), 0);

        assert_eq!(a[&1], "a");
        assert_eq!(a[&2], "b");
        assert_eq!(a[&3], "d");
        assert_eq!(a[&4], "e");
        assert_eq!(a[&5], "f");
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_963_0()
}
