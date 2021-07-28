#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_drain_rs_17_0() {
        let mut v = vec![0, 1, 2];
        let iter: std::vec::Drain<_> = v.drain(..);
    }
    _doctest_main_library_alloc_src_vec_drain_rs_17_0()
}
