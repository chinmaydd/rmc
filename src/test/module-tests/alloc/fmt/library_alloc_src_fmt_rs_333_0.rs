#![allow(unused_variables)]
#![deny(warnings)]
#![allow(dead_code)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_333_0() {
        use std::fmt;
        struct Foo; // our custom type
        impl fmt::Display for Foo {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "testing, testing")
            }
        }
    }
    _doctest_main_library_alloc_src_fmt_rs_333_0()
}
