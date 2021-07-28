#![allow(unused_variables)]
#![deny(warnings)]
// 𝄞mus<invalid>ic<invalid>
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_635_0() {
        let v = &[
            0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0xDD1E, 0x0069, 0x0063, 0xD834,
        ];

        assert_eq!(
            String::from("𝄞mus\u{FFFD}ic\u{FFFD}"),
            String::from_utf16_lossy(v)
        );
    }
    _doctest_main_library_alloc_src_string_rs_635_0()
}
