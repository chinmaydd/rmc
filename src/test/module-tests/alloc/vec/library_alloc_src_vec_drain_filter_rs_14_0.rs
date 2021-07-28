#![allow(unused_variables)]
#![deny(warnings)]
#![feature(drain_filter)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_drain_filter_rs_14_0() {
        let mut v = vec![0, 1, 2];
        let iter: std::vec::DrainFilter<_, _> = v.drain_filter(|x| *x % 2 == 0);
    }
    _doctest_main_library_alloc_src_vec_drain_filter_rs_14_0()
}
