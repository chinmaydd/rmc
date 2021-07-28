#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_777_0() {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();
        set.insert(Vec::<i32>::new());

        assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
        set.replace(Vec::with_capacity(10));
        assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_777_0()
}
