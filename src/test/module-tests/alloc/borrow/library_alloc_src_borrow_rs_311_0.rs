#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_borrow_rs_311_0() {
        use std::borrow::Cow;

        let s = "Hello world!";
        let cow: Cow<str> = Cow::Owned(String::from(s));

        assert_eq!(cow.into_owned(), String::from(s));
    }
    _doctest_main_library_alloc_src_borrow_rs_311_0()
}
