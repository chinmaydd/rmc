#![allow(unused_variables)]
#![deny(warnings)]
// 𝄞music
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_592_0() {
        let v = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];
        assert_eq!(String::from("𝄞music"), String::from_utf16(v).unwrap());

        // 𝄞mu<invalid>ic
        let v = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
        assert!(String::from_utf16(v).is_err());
    }
    _doctest_main_library_alloc_src_string_rs_592_0()
}
