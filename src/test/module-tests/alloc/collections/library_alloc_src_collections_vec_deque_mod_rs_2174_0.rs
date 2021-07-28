#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2174_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();

        buf.push_back(2);
        buf.push_back(1);
        buf.push_front(3);

        buf.make_contiguous();
        if let (slice, &[]) = buf.as_slices() {
            // we can now be sure that `slice` contains all elements of the deque,
            // while still having immutable access to `buf`.
            assert_eq!(buf.len(), slice.len());
            assert_eq!(slice, &[3, 2, 1] as &[_]);
        }
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_2174_0()
}
