#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2768_0() {
        let mut s = String::from("abc");
        let mut drain = s.drain(..);
        assert_eq!(drain.as_str(), "abc");
        let _ = drain.next().unwrap();
        assert_eq!(drain.as_str(), "bc");
    }
    _doctest_main_library_alloc_src_string_rs_2768_0()
}
