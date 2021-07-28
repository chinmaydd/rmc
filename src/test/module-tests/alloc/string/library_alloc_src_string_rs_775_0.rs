#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_775_0() {
        let s = String::from("hello");
        let bytes = s.into_bytes();

        assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
    }
    _doctest_main_library_alloc_src_string_rs_775_0()
}
