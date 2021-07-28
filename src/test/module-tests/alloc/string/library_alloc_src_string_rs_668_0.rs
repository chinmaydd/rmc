#![allow(unused_variables)]
#![deny(warnings)]
#![feature(vec_into_raw_parts)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_668_0() {
        let s = String::from("hello");

        let (ptr, len, cap) = s.into_raw_parts();

        let rebuilt = unsafe { String::from_raw_parts(ptr, len, cap) };
        assert_eq!(rebuilt, "hello");
    }
    _doctest_main_library_alloc_src_string_rs_668_0()
}
