#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_504_0() {
        let s: Box<[i32]> = Box::new([10, 40, 30]);
        let x = s.into_vec();
        // `s` cannot be used anymore because it has been converted into `x`.

        assert_eq!(x, vec![10, 40, 30]);
    }
    _doctest_main_library_alloc_src_slice_rs_504_0()
}
