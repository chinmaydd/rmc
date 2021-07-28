#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2579_0() {
        use std::borrow::Cow;
        // If the string is not owned...
        let cow: Cow<str> = Cow::Borrowed("eggplant");
        // It will allocate on the heap and copy the string.
        let owned: String = String::from(cow);
        assert_eq!(&owned[..], "eggplant");
    }
    _doctest_main_library_alloc_src_string_rs_2579_0()
}
