#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_602_0() {
        assert_eq!(["hello", "world"].concat(), "helloworld");
        assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
    }
    _doctest_main_library_alloc_src_slice_rs_602_0()
}
