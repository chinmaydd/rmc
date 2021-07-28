#![allow(unused_variables)]
#![deny(warnings)]
#![feature(binary_heap_as_slice)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_989_0() {
        use std::collections::BinaryHeap;
        use std::io::{self, Write};

        let heap = BinaryHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);

        io::sink().write(heap.as_slice()).unwrap();
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_989_0()
}
