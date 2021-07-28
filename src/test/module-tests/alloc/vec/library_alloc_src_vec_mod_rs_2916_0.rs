#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2916_0() {
        use std::convert::TryInto;
        assert_eq!(vec![1, 2, 3].try_into(), Ok([1, 2, 3]));
        assert_eq!(<Vec<i32>>::new().try_into(), Ok([]));
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2916_0()
}
