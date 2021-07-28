#![allow(unused_variables)]
#![deny(warnings)]
#![feature(btree_drain_filter)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_1201_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
        let evens: BTreeMap<_, _> = map.drain_filter(|k, _v| k % 2 == 0).collect();
        let odds = map;
        assert_eq!(evens.keys().copied().collect::<Vec<_>>(), vec![0, 2, 4, 6]);
        assert_eq!(odds.keys().copied().collect::<Vec<_>>(), vec![1, 3, 5, 7]);
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_1201_0()
}
