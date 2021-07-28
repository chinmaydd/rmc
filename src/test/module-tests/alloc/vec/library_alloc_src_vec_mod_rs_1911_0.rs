#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1911_0() {
        let mut vec = vec![1, 2, 3];
        vec.resize_with(5, Default::default);
        assert_eq!(vec, [1, 2, 3, 0, 0]);

        let mut vec = vec![];
        let mut p = 1;
        vec.resize_with(4, || {
            p *= 2;
            p
        });
        assert_eq!(vec, [2, 4, 8, 16]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1911_0()
}
