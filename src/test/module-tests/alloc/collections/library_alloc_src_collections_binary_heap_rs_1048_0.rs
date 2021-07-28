#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_1048_0() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();

        assert!(heap.is_empty());

        heap.push(3);
        heap.push(5);
        heap.push(1);

        assert!(!heap.is_empty());
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_1048_0()
}
