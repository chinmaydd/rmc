#![allow(unused_variables)]
#![deny(warnings)]
// this will panic at runtime
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_534_0() {
        b"0123456789abcdef".repeat(usize::MAX);
    }
    _doctest_main_library_alloc_src_slice_rs_534_0()
}
