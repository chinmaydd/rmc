#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1239_0() {
        let mut s = String::from("foo");

        assert_eq!(s.remove(0), 'f');
        assert_eq!(s.remove(1), 'o');
        assert_eq!(s.remove(0), 'o');
    }
    _doctest_main_library_alloc_src_string_rs_1239_0()
}
