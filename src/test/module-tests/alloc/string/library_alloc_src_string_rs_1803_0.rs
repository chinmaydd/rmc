#![allow(unused_variables)]
#![deny(warnings)]
// some invalid bytes, in a vector
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1803_0() {
        let bytes = vec![0, 159];

        let error = String::from_utf8(bytes).unwrap_err().utf8_error();

        // the first byte is invalid here
        assert_eq!(1, error.valid_up_to());
    }
    _doctest_main_library_alloc_src_string_rs_1803_0()
}
