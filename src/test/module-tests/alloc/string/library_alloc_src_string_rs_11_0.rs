#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_11_0() {
        let s = "Hello".to_string();

        let s = String::from("world");
        let s: String = "also this".into();
    }
    _doctest_main_library_alloc_src_string_rs_11_0()
}
