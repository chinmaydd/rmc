#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_476_0() {
        let string = String::from("birthday gift");
        let boxed_str = string.clone().into_boxed_str();

        assert_eq!(boxed_str.into_string(), string);
    }
    _doctest_main_library_alloc_src_str_rs_476_0()
}
