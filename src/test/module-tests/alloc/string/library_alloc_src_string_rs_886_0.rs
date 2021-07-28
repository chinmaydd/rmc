#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_886_0() {
        let s = String::with_capacity(10);

        assert!(s.capacity() >= 10);
    }
    _doctest_main_library_alloc_src_string_rs_886_0()
}
