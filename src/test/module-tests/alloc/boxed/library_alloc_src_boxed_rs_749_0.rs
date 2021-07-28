#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_749_0() {
        let x = Box::new(5);
        let ptr = Box::into_raw(x);
        let x = unsafe { Box::from_raw(ptr) };
    }
    _doctest_main_library_alloc_src_boxed_rs_749_0()
}
