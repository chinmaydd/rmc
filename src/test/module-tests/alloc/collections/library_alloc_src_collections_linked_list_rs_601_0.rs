#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_601_0() {
        use std::collections::LinkedList;

        let mut dl = LinkedList::new();

        dl.push_front(2);
        dl.push_front(1);
        assert_eq!(dl.len(), 2);
        assert_eq!(dl.front(), Some(&1));

        dl.clear();
        assert_eq!(dl.len(), 0);
        assert_eq!(dl.front(), None);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_601_0()
}
