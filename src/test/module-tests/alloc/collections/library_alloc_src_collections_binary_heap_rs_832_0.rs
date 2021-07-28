#![allow(unused_variables)]
#![deny(warnings)]
#![feature(binary_heap_into_iter_sorted)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_832_0() {
        use std::collections::BinaryHeap;
        let heap = BinaryHeap::from(vec![1, 2, 3, 4, 5]);

        assert_eq!(
            heap.into_iter_sorted().take(2).collect::<Vec<_>>(),
            vec![5, 4]
        );
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_832_0()
}
