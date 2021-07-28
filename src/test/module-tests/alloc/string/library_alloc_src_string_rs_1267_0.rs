#![allow(unused_variables)]
#![deny(warnings)]
#![feature(string_remove_matches)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1267_0() {
        let mut s = String::from("Trees are not green, the sky is not blue.");
        s.remove_matches("not ");
        assert_eq!("Trees are green, the sky is blue.", s);
    }
    _doctest_main_library_alloc_src_string_rs_1267_0()
}
