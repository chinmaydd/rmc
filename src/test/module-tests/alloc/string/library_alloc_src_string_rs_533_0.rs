#![allow(unused_variables)]
#![deny(warnings)]
// some bytes, in a vector
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_533_0() {
        let sparkle_heart = vec![240, 159, 146, 150];

        let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);

        assert_eq!("💖", sparkle_heart);
    }
    _doctest_main_library_alloc_src_string_rs_533_0()
}
