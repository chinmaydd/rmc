#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_767_0() {
        use std::collections::LinkedList;

        let mut d = LinkedList::new();
        assert_eq!(d.pop_front(), None);

        d.push_front(1);
        d.push_front(3);
        assert_eq!(d.pop_front(), Some(3));
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_front(), None);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_767_0()
}
