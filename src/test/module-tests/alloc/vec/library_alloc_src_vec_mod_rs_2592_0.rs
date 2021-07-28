#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2592_0() {
        let mut v = vec![1, 2, 3];
        let new = [7, 8];
        let u: Vec<_> = v.splice(..2, new).collect();
        assert_eq!(v, &[7, 8, 3]);
        assert_eq!(u, &[1, 2]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2592_0()
}
