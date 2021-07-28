#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_binary_heap_rs_218_0() {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();

        // Wrap values in `Reverse`
        heap.push(Reverse(1));
        heap.push(Reverse(5));
        heap.push(Reverse(2));

        // If we pop these scores now, they should come back in the reverse order.
        assert_eq!(heap.pop(), Some(Reverse(1)));
        assert_eq!(heap.pop(), Some(Reverse(2)));
        assert_eq!(heap.pop(), Some(Reverse(5)));
        assert_eq!(heap.pop(), None);
    }
    _doctest_main_library_alloc_src_collections_binary_heap_rs_218_0()
}
