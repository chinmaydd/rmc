#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_941_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
        // Keep only the elements with even-numbered keys.
        map.retain(|&k, _| k % 2 == 0);
        assert!(map.into_iter().eq(vec![(0, 0), (2, 20), (4, 40), (6, 60)]));
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_941_0()
}
