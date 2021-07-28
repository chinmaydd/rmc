#![allow(unused_variables)]
#![deny(warnings)]
// slicing a Vec
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_8_0() {
        let vec = vec![1, 2, 3];
        let int_slice = &vec[..];
        // coercing an array to a slice
        let str_slice: &[&str] = &["one", "two", "three"];
    }
    _doctest_main_library_alloc_src_slice_rs_8_0()
}
