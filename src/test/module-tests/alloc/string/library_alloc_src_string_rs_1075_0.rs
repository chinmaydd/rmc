#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1075_0() {
        let mut s = String::from("foo");

        s.reserve(100);
        assert!(s.capacity() >= 100);

        s.shrink_to_fit();
        assert_eq!(3, s.capacity());
    }
    _doctest_main_library_alloc_src_string_rs_1075_0()
}
