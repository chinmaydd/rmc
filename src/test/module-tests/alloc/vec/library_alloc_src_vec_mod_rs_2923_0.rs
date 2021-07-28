#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2923_0() {
        use std::convert::TryInto;
        let r: Result<[i32; 4], _> = (0..10).collect::<Vec<_>>().try_into();
        assert_eq!(r, Err(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2923_0()
}
