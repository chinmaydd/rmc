#![allow(unused_variables)]
#![deny(warnings)]
// some invalid bytes, in a vector
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_479_0() {
        let sparkle_heart = vec![0, 159, 146, 150];

        assert!(String::from_utf8(sparkle_heart).is_err());
    }
    _doctest_main_library_alloc_src_string_rs_479_0()
}
