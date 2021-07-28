#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1538_0() {
        let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];

        vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));

        assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1538_0()
}
