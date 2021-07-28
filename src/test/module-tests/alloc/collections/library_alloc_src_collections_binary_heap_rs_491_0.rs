#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_491_0() {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(vec![1, 2, 4, 5, 7]);
        heap.push(6);
        heap.push(3);

        let vec = heap.into_sorted_vec();
        assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_491_0()
}
