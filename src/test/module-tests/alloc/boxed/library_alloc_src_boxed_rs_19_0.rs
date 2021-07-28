#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_19_0() {
        let boxed: Box<u8> = Box::new(5);
        let val: u8 = *boxed;
    }
    _doctest_main_library_alloc_src_boxed_rs_19_0()
}
