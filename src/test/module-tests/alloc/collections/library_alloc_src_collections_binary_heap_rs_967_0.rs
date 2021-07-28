#![allow(unused_variables)]
#![deny(warnings)]
#![feature(shrink_to)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_967_0() {
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);

        assert!(heap.capacity() >= 100);
        heap.shrink_to(10);
        assert!(heap.capacity() >= 10);
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_967_0()
}
