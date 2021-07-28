#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_274_0() {
        assert_eq!(format!("Hello {{}}"), "Hello {}");
        assert_eq!(format!("{{ Hello"), "{ Hello");
    }
    _doctest_main_library_alloc_src_fmt_rs_274_0()
}
