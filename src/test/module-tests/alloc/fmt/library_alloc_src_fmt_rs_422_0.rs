#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_422_0() {
        assert_eq!(format!("{} {:?}", 3, 4), "3 4");
        assert_eq!(format!("{} {:?}", 'a', 'b'), "a 'b'");
        assert_eq!(format!("{} {:?}", "foo\n", "bar\n"), "foo\n \"bar\\n\"");
    }
    _doctest_main_library_alloc_src_fmt_rs_422_0()
}
