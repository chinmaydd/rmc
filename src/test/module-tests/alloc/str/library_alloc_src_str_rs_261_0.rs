#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_261_0() {
        let s = "this is old";

        assert_eq!("this is new", s.replace("old", "new"));
    }
    _doctest_main_library_alloc_src_str_rs_261_0()
}
