#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2556_0() {
        let s1: String = String::from("hello world");
        let s2: Box<str> = Box::from(s1);
        let s3: String = String::from(s2);

        assert_eq!("hello world", s3)
    }
    _doctest_main_library_alloc_src_string_rs_2556_0()
}
