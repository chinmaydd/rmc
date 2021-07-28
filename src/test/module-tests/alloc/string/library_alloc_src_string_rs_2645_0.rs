#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2645_0() {
        use std::borrow::Cow;
        let s = "eggplant".to_string();
        assert_eq!(Cow::from(&s), Cow::Borrowed("eggplant"));
    }
    _doctest_main_library_alloc_src_string_rs_2645_0()
}
