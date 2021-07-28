#![allow(unused_variables)]
#![deny(warnings)]
#![allow(deprecated)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_637_0() {
        assert_eq!(["hello", "world"].connect(" "), "hello world");
        assert_eq!([[1, 2], [3, 4]].connect(&0), [1, 2, 0, 3, 4]);
    }
    _doctest_main_library_alloc_src_slice_rs_637_0()
}
