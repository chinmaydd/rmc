#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_744_0() {
        use std::collections::LinkedList;

        let mut dl = LinkedList::new();

        dl.push_front(2);
        assert_eq!(dl.front().unwrap(), &2);

        dl.push_front(1);
        assert_eq!(dl.front().unwrap(), &1);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_744_0()
}
