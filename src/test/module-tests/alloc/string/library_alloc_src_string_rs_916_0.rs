#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_916_0() {
        let mut s = String::new();

        s.reserve(10);

        assert!(s.capacity() >= 10);
    }
    _doctest_main_library_alloc_src_string_rs_916_0()
}
