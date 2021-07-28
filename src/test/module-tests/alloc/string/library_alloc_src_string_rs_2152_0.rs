#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2152_0() {
        let a = String::from("hello");
        let b = String::from(" world");
        let c = a + &b;
        // `a` is moved and can no longer be used here.
    }
    _doctest_main_library_alloc_src_string_rs_2152_0()
}
