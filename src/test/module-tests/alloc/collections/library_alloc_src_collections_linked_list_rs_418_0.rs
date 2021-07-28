#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_418_0() {
        use std::collections::LinkedList;

        let mut list1 = LinkedList::new();
        list1.push_back('a');

        let mut list2 = LinkedList::new();
        list2.push_back('b');
        list2.push_back('c');

        list1.append(&mut list2);

        let mut iter = list1.iter();
        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.next(), Some(&'c'));
        assert!(iter.next().is_none());

        assert!(list2.is_empty());
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_418_0()
}
