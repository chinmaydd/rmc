#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_1073_0() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::from(vec![1, 3]);

        assert!(!heap.is_empty());

        for x in heap.drain() {
            println!("{}", x);
        }

        assert!(heap.is_empty());
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_1073_0()
}
