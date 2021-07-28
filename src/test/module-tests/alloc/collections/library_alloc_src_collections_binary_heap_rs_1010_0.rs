#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_1010_0() {
        use std::collections::BinaryHeap;
        let heap = BinaryHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);
        let vec = heap.into_vec();

        // Will print in some order
        for x in vec {
            println!("{}", x);
        }
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_1010_0()
}
