#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1368_0() {
        let mut v = vec![1, 2, 3];
        assert_eq!(v.remove(1), 2);
        assert_eq!(v, [1, 3]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1368_0()
}
