#![allow(unused_variables)]
#![deny(warnings)]
#![feature(binary_heap_retain)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_773_0() {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(vec![-10, -5, 1, 2, 4, 13]);

        heap.retain(|x| x % 2 == 0); // only keep even numbers

        assert_eq!(heap.into_sorted_vec(), [-10, 2, 4])
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_773_0()
}
