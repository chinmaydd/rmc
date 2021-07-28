#![allow(unused_variables)]
#![deny(warnings)]
// some bytes, in a vector
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_106_0() {
        let sparkle_heart = vec![240, 159, 146, 150];

        // We know these bytes are valid, so we'll use `unwrap()`.
        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

        assert_eq!("💖", sparkle_heart);
    }
    _doctest_main_library_alloc_src_string_rs_106_0()
}
