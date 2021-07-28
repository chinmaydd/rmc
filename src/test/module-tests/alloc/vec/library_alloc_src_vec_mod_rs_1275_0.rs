#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1275_0() {
        let mut v = vec!["foo", "bar", "baz", "qux"];

        assert_eq!(v.swap_remove(1), "bar");
        assert_eq!(v, ["foo", "qux", "baz"]);

        assert_eq!(v.swap_remove(0), "foo");
        assert_eq!(v, ["baz", "qux"]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1275_0()
}
