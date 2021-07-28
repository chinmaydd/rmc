#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2831_0() {
        let c: char = 'a';
        let s: String = String::from(c);
        assert_eq!("a", &s[..]);
    }
    _doctest_main_library_alloc_src_string_rs_2831_0()
}
