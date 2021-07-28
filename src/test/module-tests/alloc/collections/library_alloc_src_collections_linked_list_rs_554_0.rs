#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_554_0() {
        use std::collections::LinkedList;

        let mut dl = LinkedList::new();
        assert!(dl.is_empty());

        dl.push_front("foo");
        assert!(!dl.is_empty());
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_554_0()
}
