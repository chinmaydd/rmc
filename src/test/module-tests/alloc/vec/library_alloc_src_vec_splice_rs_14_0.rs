#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_splice_rs_14_0() {
        let mut v = vec![0, 1, 2];
        let new = [7, 8];
        let iter: std::vec::Splice<_> = v.splice(1.., new);
    }
    _doctest_main_library_alloc_src_vec_splice_rs_14_0()
}
