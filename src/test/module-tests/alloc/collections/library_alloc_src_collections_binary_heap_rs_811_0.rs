#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_811_0() {
        use std::collections::BinaryHeap;
        let heap = BinaryHeap::from(vec![1, 2, 3, 4]);

        // Print 1, 2, 3, 4 in arbitrary order
        for x in heap.iter() {
            println!("{}", x);
        }
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_811_0()
}
