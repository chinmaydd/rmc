#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1355_0() {
        let mut s = String::from("abcde");
        let keep = [false, true, true, false, true];
        let mut i = 0;
        s.retain(|_| (keep[i], i += 1).0);
        assert_eq!(s, "bce");
    }
    _doctest_main_library_alloc_src_string_rs_1355_0()
}
