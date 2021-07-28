#![allow(unused_variables)]
#![deny(warnings)]
// some invalid bytes, in a vector
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1753_0() {
        let bytes = vec![0, 159];

        let value = String::from_utf8(bytes);

        assert_eq!(&[0, 159], value.unwrap_err().as_bytes());
    }
    _doctest_main_library_alloc_src_string_rs_1753_0()
}
