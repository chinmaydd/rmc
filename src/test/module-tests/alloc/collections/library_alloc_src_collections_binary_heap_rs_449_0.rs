#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_449_0() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        heap.push(3);
        heap.push(5);
        heap.push(1);

        assert_eq!(heap.len(), 3);
        assert_eq!(heap.peek(), Some(&5));
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_449_0()
}
