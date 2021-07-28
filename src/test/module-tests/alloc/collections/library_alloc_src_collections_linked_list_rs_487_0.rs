#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_487_0() {
        use std::collections::LinkedList;

        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);

        for element in list.iter_mut() {
            *element += 10;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), Some(&12));
        assert_eq!(iter.next(), None);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_487_0()
}
