#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_830_0() {
        use std::collections::BTreeSet;

        let mut set: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(set.take(&2), Some(2));
        assert_eq!(set.take(&2), None);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_830_0()
}
