#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_559_0() {
        let s = "Grüße, Jürgen ❤";

        assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());
    }
    _doctest_main_library_alloc_src_str_rs_559_0()
}
