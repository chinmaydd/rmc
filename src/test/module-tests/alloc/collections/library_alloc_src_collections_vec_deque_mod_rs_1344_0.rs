#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1344_0() {
        use std::collections::VecDeque;

        let mut d = VecDeque::new();
        assert_eq!(d.back(), None);

        d.push_back(1);
        d.push_back(2);
        match d.back_mut() {
            Some(x) => *x = 9,
            None => (),
        }
        assert_eq!(d.back(), Some(&9));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_1344_0()
}
