#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1471_0() {
        let mut s = String::from("bar");

        s.insert_str(0, "foo");

        assert_eq!("foobar", s);
    }
    _doctest_main_library_alloc_src_string_rs_1471_0()
}
