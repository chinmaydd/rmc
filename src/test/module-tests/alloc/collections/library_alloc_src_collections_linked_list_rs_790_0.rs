#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_linked_list_rs_790_0() {
        use std::collections::LinkedList;

        let mut d = LinkedList::new();
        d.push_back(1);
        d.push_back(3);
        assert_eq!(3, *d.back().unwrap());
    }
    _doctest_main_library_alloc_src_collections_linked_list_rs_790_0()
}
