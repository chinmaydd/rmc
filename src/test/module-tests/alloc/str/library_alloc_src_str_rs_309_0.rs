#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_309_0() {
        let s = "this is old";
        assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
    }
    _doctest_main_library_alloc_src_str_rs_309_0()
}
