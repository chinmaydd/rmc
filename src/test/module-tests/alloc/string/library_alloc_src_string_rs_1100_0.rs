#![allow(unused_variables)]
#![deny(warnings)]
#![feature(shrink_to)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1100_0() {
        let mut s = String::from("foo");

        s.reserve(100);
        assert!(s.capacity() >= 100);

        s.shrink_to(10);
        assert!(s.capacity() >= 10);
        s.shrink_to(0);
        assert!(s.capacity() >= 3);
    }
    _doctest_main_library_alloc_src_string_rs_1100_0()
}
