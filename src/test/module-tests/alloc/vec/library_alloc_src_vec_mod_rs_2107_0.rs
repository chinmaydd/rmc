#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2107_0() {
        let mut vec = vec!["hello"];
        vec.resize(3, "world");
        assert_eq!(vec, ["hello", "world", "world"]);

        let mut vec = vec![1, 2, 3, 4];
        vec.resize(2, 0);
        assert_eq!(vec, [1, 2]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2107_0()
}
