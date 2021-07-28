#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_77_0() {
        format!("{argument}", argument = "test"); // => "test"
        format!("{name} {}", 1, name = 2); // => "2 1"
        format!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
    }
    _doctest_main_library_alloc_src_fmt_rs_77_0()
}
