#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_835_0() {
        use std::collections::LinkedList;

        let mut d = LinkedList::new();

        d.push_front(1);
        d.push_front(2);
        d.push_front(3);

        let mut split = d.split_off(2);

        assert_eq!(split.pop_front(), Some(1));
        assert_eq!(split.pop_front(), None);
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_835_0()
}
