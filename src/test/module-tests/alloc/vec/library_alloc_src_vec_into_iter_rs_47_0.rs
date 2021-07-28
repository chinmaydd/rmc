#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_into_iter_rs_47_0() {
        let vec = vec!['a', 'b', 'c'];
        let mut into_iter = vec.into_iter();
        assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
        let _ = into_iter.next().unwrap();
        assert_eq!(into_iter.as_slice(), &['b', 'c']);
    }
    _doctest_main_library_alloc_src_vec_into_iter_rs_47_0()
}
