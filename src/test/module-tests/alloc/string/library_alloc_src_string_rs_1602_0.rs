#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1602_0() {
        let mut s = String::from("foo");

        s.clear();

        assert!(s.is_empty());
        assert_eq!(0, s.len());
        assert_eq!(3, s.capacity());
    }
    _doctest_main_library_alloc_src_string_rs_1602_0()
}
