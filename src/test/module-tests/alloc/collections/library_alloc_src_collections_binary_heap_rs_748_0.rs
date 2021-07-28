#![allow(unused_variables)]
#![deny(warnings)]
#![feature(binary_heap_drain_sorted)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_748_0() {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(heap.len(), 5);

        drop(heap.drain_sorted()); // removes all elements in heap order
        assert_eq!(heap.len(), 0);
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_748_0()
}
