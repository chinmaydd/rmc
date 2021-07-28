#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_561_0() {
        use std::fmt;

        let s = fmt::format(format_args!("Hello, {}!", "world"));
        assert_eq!(s, "Hello, world!");
    }
    _doctest_main_library_alloc_src_fmt_rs_561_0()
}
