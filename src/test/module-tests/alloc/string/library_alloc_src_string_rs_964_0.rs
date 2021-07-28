#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_964_0() {
        let mut s = String::new();

        s.reserve_exact(10);

        assert!(s.capacity() >= 10);
    }
    _doctest_main_library_alloc_src_string_rs_964_0()
}
