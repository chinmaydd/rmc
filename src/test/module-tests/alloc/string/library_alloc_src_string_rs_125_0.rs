#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_125_0() {
        let s = "hello";

        println!("The first letter of s is {}", s[0]); // ERROR!!!
    }
    _doctest_main_library_alloc_src_string_rs_125_0()
}
