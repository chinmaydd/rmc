#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_152_0() {
        assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");
        assert_eq!(format!("{:#x}!", 27), "0x1b!");
        assert_eq!(format!("Hello {:05}!", 5), "Hello 00005!");
        assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!");
        assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
    }
    _doctest_main_library_alloc_src_fmt_rs_152_0()
}
