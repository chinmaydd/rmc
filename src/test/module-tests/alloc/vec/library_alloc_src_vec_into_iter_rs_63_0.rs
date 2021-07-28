#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_into_iter_rs_63_0() {
        let vec = vec!['a', 'b', 'c'];
        let mut into_iter = vec.into_iter();
        assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
        into_iter.as_mut_slice()[2] = 'z';
        assert_eq!(into_iter.next().unwrap(), 'a');
        assert_eq!(into_iter.next().unwrap(), 'b');
        assert_eq!(into_iter.next().unwrap(), 'z');
    }
    _doctest_main_library_alloc_src_vec_into_iter_rs_63_0()
}
