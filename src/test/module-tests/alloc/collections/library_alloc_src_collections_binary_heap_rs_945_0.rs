#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_945_0() {
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);

        assert!(heap.capacity() >= 100);
        heap.shrink_to_fit();
        assert!(heap.capacity() == 0);
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_945_0()
}
