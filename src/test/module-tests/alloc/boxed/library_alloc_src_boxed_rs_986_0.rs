#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_986_0() {
        let x = vec![1, 2, 3].into_boxed_slice();
        let static_ref = Box::leak(x);
        static_ref[0] = 4;
        assert_eq!(*static_ref, [4, 2, 3]);
    }
    _doctest_main_library_alloc_src_boxed_rs_986_0()
}
