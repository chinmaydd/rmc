#![allow(unused_variables)]
#![deny(warnings)]
#![feature(string_remove_matches)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1277_0() {
        let mut s = String::from("banana");
        s.remove_matches("ana");
        assert_eq!("bna", s);
    }
    _doctest_main_library_alloc_src_string_rs_1277_0()
}
