#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_239_0() {
        let s = "this is a string";
        let boxed_str = s.to_owned().into_boxed_str();
        let boxed_bytes = boxed_str.into_boxed_bytes();
        assert_eq!(*boxed_bytes, *s.as_bytes());
    }
    _doctest_main_library_alloc_src_str_rs_239_0()
}
