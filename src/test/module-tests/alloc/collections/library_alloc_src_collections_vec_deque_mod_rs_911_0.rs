#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_911_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(5);
        buf.push_back(3);
        buf.push_back(4);
        let b: &[_] = &[&5, &3, &4];
        let c: Vec<&i32> = buf.iter().collect();
        assert_eq!(&c[..], b);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_911_0()
}
