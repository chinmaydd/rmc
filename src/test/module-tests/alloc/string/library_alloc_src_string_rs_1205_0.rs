#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1205_0() {
        let mut s = String::from("foo");

        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('f'));

        assert_eq!(s.pop(), None);
    }
    _doctest_main_library_alloc_src_string_rs_1205_0()
}
