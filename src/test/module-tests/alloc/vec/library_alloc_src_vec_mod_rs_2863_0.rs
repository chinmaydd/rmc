#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2863_0() {
        let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
        assert_eq!(Vec::from(b), vec![1, 2, 3]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2863_0()
}
