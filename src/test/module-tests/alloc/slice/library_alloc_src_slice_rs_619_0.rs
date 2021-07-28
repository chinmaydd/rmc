#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_619_0() {
        assert_eq!(["hello", "world"].join(" "), "hello world");
        assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
        assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
    }
    _doctest_main_library_alloc_src_slice_rs_619_0()
}
