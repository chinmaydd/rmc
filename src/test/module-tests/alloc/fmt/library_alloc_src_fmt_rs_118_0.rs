#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_118_0() {
        assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
        assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
        assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
        assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    }
    _doctest_main_library_alloc_src_fmt_rs_118_0()
}
