#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_239_0() {
        println!(
            "{}, `{name:.*}` has 3 fractional digits",
            "Hello",
            3,
            name = 1234.56
        );
        println!(
            "{}, `{name:.*}` has 3 characters",
            "Hello",
            3,
            name = "1234.56"
        );
        println!(
            "{}, `{name:>8.*}` has 3 right-aligned characters",
            "Hello",
            3,
            name = "1234.56"
        );
    }
    _doctest_main_library_alloc_src_fmt_rs_239_0()
}
