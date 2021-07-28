#![allow(unused_variables)]
#![deny(warnings)]
#![feature(linked_list_cursors)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_1631_0() {
        use std::collections::LinkedList;
        let mut dl = LinkedList::new();
        dl.push_front(3);
        dl.push_front(2);
        dl.push_front(1);
        let mut cursor = dl.cursor_front_mut();
        *cursor.current().unwrap() = 99;
        *cursor.back_mut().unwrap() = 0;
        let mut contents = dl.into_iter();
        assert_eq!(contents.next(), Some(99));
        assert_eq!(contents.next(), Some(2));
        assert_eq!(contents.next(), Some(0));
        assert_eq!(contents.next(), None);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_1631_0()
}
