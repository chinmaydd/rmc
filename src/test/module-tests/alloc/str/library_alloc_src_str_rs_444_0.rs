#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_444_0() {
        let s = "tschüß";

        assert_eq!("TSCHÜSS", s.to_uppercase());
    }
    _doctest_main_library_alloc_src_str_rs_444_0()
}
