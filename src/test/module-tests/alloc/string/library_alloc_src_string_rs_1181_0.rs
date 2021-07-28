#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1181_0() {
        let mut s = String::from("hello");

        s.truncate(2);

        assert_eq!("he", s);
    }
    _doctest_main_library_alloc_src_string_rs_1181_0()
}
