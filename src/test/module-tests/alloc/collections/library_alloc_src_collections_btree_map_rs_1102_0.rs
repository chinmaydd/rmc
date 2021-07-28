#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_1102_0() {
        use std::collections::BTreeMap;

        let mut count: BTreeMap<&str, usize> = BTreeMap::new();

        // count the number of occurrences of letters in the vec
        for x in vec!["a", "b", "a", "c", "a", "b"] {
            *count.entry(x).or_insert(0) += 1;
        }

        assert_eq!(count["a"], 3);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_1102_0()
}
