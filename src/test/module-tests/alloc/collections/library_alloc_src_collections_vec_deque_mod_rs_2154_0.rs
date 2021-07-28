#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2154_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::with_capacity(15);

        buf.push_back(2);
        buf.push_back(1);
        buf.push_front(3);

        // sorting the deque
        buf.make_contiguous().sort();
        assert_eq!(buf.as_slices(), (&[1, 2, 3] as &[_], &[] as &[_]));

        // sorting it in reverse order
        buf.make_contiguous().sort_by(|a, b| b.cmp(a));
        assert_eq!(buf.as_slices(), (&[3, 2, 1] as &[_], &[] as &[_]));
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2154_0()
}
