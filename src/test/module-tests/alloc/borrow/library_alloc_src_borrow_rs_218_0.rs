#![allow(unused_variables)]
#![deny(warnings)]
#![feature(cow_is_borrowed)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_borrow_rs_218_0() {
        use std::borrow::Cow;

        let cow = Cow::Borrowed("moo");
        assert!(cow.is_borrowed());

        let bull: Cow<'_, str> = Cow::Owned("...moo?".to_string());
        assert!(!bull.is_borrowed());
    }
    _doctest_main_library_alloc_src_borrow_rs_218_0()
}
