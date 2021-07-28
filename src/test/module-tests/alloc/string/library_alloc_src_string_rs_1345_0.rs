#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1345_0() {
        let mut s = String::from("f_o_ob_ar");

        s.retain(|c| c != '_');

        assert_eq!(s, "foobar");
    }
    _doctest_main_library_alloc_src_string_rs_1345_0()
}
