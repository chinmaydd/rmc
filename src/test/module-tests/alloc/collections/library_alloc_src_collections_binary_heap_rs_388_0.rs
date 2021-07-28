#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_388_0() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        assert!(heap.peek_mut().is_none());

        heap.push(1);
        heap.push(5);
        heap.push(2);
        {
            let mut val = heap.peek_mut().unwrap();
            *val = 0;
        }
        assert_eq!(heap.peek(), Some(&2));
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_388_0()
}
