#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_709_0() {
        use std::collections::BinaryHeap;

        let v = vec![-10, 1, 2, 3, 3];
        let mut a = BinaryHeap::from(v);

        let v = vec![-20, 5, 43];
        let mut b = BinaryHeap::from(v);

        a.append(&mut b);

        assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
        assert!(b.is_empty());
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_709_0()
}
