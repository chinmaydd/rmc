#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_575_0() {
        use std::collections::LinkedList;

        let mut dl = LinkedList::new();

        dl.push_front(2);
        assert_eq!(dl.len(), 1);

        dl.push_front(1);
        assert_eq!(dl.len(), 2);

        dl.push_back(3);
        assert_eq!(dl.len(), 3);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_575_0()
}
