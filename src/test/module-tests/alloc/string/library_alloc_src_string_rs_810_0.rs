#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_810_0() {
        let mut s = String::from("foobar");
        let s_mut_str = s.as_mut_str();

        s_mut_str.make_ascii_uppercase();

        assert_eq!("FOOBAR", s_mut_str);
    }
    _doctest_main_library_alloc_src_string_rs_810_0()
}
