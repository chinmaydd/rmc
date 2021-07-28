#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_528_0() {
        let s = "Grüße, Jürgen ❤";

        assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());
    }
    _doctest_main_library_alloc_src_str_rs_528_0()
}
