#![allow(unused_variables)]
#![deny(warnings)]
#![feature(string_extend_from_within)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_853_0() {
        let mut string = String::from("abcde");

        string.extend_from_within(2..);
        assert_eq!(string, "abcdecde");

        string.extend_from_within(..2);
        assert_eq!(string, "abcdecdeab");

        string.extend_from_within(4..8);
        assert_eq!(string, "abcdecdeabecde");
    }
    _doctest_main_library_alloc_src_string_rs_853_0()
}
