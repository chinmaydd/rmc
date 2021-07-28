#![allow(unused_variables)]
#![deny(warnings)]
#![allow(unused_must_use)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_452_0() {
        use std::io::Write;
        let mut w = Vec::new();
        write!(&mut w, "Hello {}!", "world");
    }
    _doctest_main_library_alloc_src_fmt_rs_452_0()
}
