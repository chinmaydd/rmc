#![allow(unused_variables)]
#![deny(warnings)]
#![feature(cow_is_borrowed)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_borrow_rs_241_0() {
        use std::borrow::Cow;

        let cow: Cow<'_, str> = Cow::Owned("moo".to_string());
        assert!(cow.is_owned());

        let bull = Cow::Borrowed("...moo?");
        assert!(!bull.is_owned());
    }
    _doctest_main_library_alloc_src_borrow_rs_241_0()
}
