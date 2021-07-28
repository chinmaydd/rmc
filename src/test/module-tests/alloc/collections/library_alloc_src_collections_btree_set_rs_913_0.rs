#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_913_0() {
        use std::collections::BTreeSet;

        let mut a = BTreeSet::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);
        a.insert(17);
        a.insert(41);

        let b = a.split_off(&3);

        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 3);

        assert!(a.contains(&1));
        assert!(a.contains(&2));

        assert!(b.contains(&3));
        assert!(b.contains(&17));
        assert!(b.contains(&41));
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_913_0()
}
