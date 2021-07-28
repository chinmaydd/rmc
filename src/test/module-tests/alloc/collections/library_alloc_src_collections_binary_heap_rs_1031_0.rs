#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_1031_0() {
        use std::collections::BinaryHeap;
        let heap = BinaryHeap::from(vec![1, 3]);

        assert_eq!(heap.len(), 2);
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_1031_0()
}
