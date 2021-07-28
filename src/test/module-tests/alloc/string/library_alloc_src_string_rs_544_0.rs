#![allow(unused_variables)]
#![deny(warnings)]
// some invalid bytes
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_544_0() {
        let input = b"Hello \xF0\x90\x80World";
        let output = String::from_utf8_lossy(input);

        assert_eq!("Hello �World", output);
    }
    _doctest_main_library_alloc_src_string_rs_544_0()
}
