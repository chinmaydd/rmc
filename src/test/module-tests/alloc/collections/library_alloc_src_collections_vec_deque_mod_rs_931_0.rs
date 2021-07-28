#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_931_0() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();
        buf.push_back(5);
        buf.push_back(3);
        buf.push_back(4);
        for num in buf.iter_mut() {
            *num = *num - 2;
        }
        let b: &[_] = &[&mut 3, &mut 1, &mut 2];
        assert_eq!(&buf.iter_mut().collect::<Vec<&mut i32>>()[..], b);
    }
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_931_0()
}
