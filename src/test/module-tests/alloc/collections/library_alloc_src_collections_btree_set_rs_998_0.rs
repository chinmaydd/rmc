#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_998_0() {
        use std::collections::BTreeSet;

        let set: BTreeSet<usize> = [3, 1, 2].iter().cloned().collect();
        let mut set_iter = set.iter();
        assert_eq!(set_iter.next(), Some(&1));
        assert_eq!(set_iter.next(), Some(&2));
        assert_eq!(set_iter.next(), Some(&3));
        assert_eq!(set_iter.next(), None);
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_998_0()
}
