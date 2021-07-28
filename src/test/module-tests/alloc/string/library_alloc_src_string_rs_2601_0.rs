#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2601_0() {
        use std::borrow::Cow;
        assert_eq!(Cow::from("eggplant"), Cow::Borrowed("eggplant"));
    }
    _doctest_main_library_alloc_src_string_rs_2601_0()
}
