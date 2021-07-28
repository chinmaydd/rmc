#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2931_0() {
        use std::convert::TryInto;
        let mut v = String::from("hello world").into_bytes();
        v.sort();
        v.truncate(2);
        let [a, b]: [_; 2] = v.try_into().unwrap();
        assert_eq!(a, b' ');
        assert_eq!(b, b'd');
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2931_0()
}
