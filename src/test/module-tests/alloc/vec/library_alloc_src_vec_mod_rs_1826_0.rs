#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1826_0() {
        let mut v = Vec::new();
        assert!(v.is_empty());

        v.push(1);
        assert!(!v.is_empty());
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1826_0()
}
