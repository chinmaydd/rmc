#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1510_0() {
        let mut vec = vec![10, 20, 21, 30, 20];

        vec.dedup_by_key(|i| *i / 10);

        assert_eq!(vec, [10, 20, 30, 20]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1510_0()
}
