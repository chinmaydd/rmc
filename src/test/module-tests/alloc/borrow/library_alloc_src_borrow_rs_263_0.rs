#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_borrow_rs_263_0() {
        use std::borrow::Cow;

        let mut cow = Cow::Borrowed("foo");
        cow.to_mut().make_ascii_uppercase();

        assert_eq!(cow, Cow::Owned(String::from("FOO")) as Cow<str>);
    }
    _doctest_main_library_alloc_src_borrow_rs_263_0()
}
