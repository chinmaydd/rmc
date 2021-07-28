#![allow(unused_variables)]
#![deny(warnings)]
#![feature(linked_list_remove)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_890_0() {
        use std::collections::LinkedList;

        let mut d = LinkedList::new();

        d.push_front(1);
        d.push_front(2);
        d.push_front(3);

        assert_eq!(d.remove(1), 2);
        assert_eq!(d.remove(0), 3);
        assert_eq!(d.remove(0), 1);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_890_0()
}
