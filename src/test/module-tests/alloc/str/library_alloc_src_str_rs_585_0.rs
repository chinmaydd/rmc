#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_585_0() {
        let smile_utf8 = Box::new([226, 152, 186]);
        let smile = unsafe { std::str::from_boxed_utf8_unchecked(smile_utf8) };

        assert_eq!("☺", &*smile);
    }
    _doctest_main_library_alloc_src_str_rs_585_0()
}
