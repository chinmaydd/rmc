#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1125_0() {
        let mut s = String::from("abc");

        s.push('1');
        s.push('2');
        s.push('3');

        assert_eq!("abc123", s);
    }
    _doctest_main_library_alloc_src_string_rs_1125_0()
}
