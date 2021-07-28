#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1527_0() {
        let a = String::from("foo");
        assert_eq!(a.len(), 3);

        let fancy_f = String::from("ƒoo");
        assert_eq!(fancy_f.len(), 4);
        assert_eq!(fancy_f.chars().count(), 3);
    }
    _doctest_main_library_alloc_src_string_rs_1527_0()
}
