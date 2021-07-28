#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1732_0() {
        let s = String::from("hello");

        let b = s.into_boxed_str();
    }
    _doctest_main_library_alloc_src_string_rs_1732_0()
}
