#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_148_0() {
        fn takes_str(s: &str) {}

        let s = String::from("Hello");

        takes_str(&s);
    }
    _doctest_main_library_alloc_src_string_rs_148_0()
}
