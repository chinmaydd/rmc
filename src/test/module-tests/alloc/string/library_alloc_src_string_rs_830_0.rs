#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_830_0() {
        let mut s = String::from("foo");

        s.push_str("bar");

        assert_eq!("foobar", s);
    }
    _doctest_main_library_alloc_src_string_rs_830_0()
}
