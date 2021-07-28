#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_852_0() {
        use std::collections::BTreeSet;

        let xs = [1, 2, 3, 4, 5, 6];
        let mut set: BTreeSet<i32> = xs.iter().cloned().collect();
        // Keep only the even numbers.
        set.retain(|&k| k % 2 == 0);
        assert!(set.iter().eq([2, 4, 6].iter()));
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_852_0()
}
