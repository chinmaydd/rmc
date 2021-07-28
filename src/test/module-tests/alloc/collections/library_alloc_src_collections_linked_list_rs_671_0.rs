#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_671_0() {
        use std::collections::LinkedList;

        let mut dl = LinkedList::new();
        assert_eq!(dl.front(), None);

        dl.push_front(1);
        assert_eq!(dl.front(), Some(&1));

        match dl.front_mut() {
            None => {}
            Some(x) => *x = 5,
        }
        assert_eq!(dl.front(), Some(&5));
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_671_0()
}
