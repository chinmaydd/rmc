#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_902_0() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        heap.reserve_exact(100);
        assert!(heap.capacity() >= 100);
        heap.push(4);
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_902_0()
}
