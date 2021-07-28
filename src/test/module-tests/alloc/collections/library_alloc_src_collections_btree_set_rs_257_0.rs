#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_257_0() {
        use std::collections::BTreeSet;
        use std::ops::Bound::Included;

        let mut set = BTreeSet::new();
        set.insert(3);
        set.insert(5);
        set.insert(8);
        for &elem in set.range((Included(&4), Included(&8))) {
            println!("{}", elem);
        }
        assert_eq!(Some(&5), set.range(4..).next());
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_257_0()
}
