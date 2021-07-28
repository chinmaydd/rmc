#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_977_0() {
        let x = Box::new(41);
        let static_ref: &'static mut usize = Box::leak(x);
        *static_ref += 1;
        assert_eq!(*static_ref, 42);
    }
    _doctest_main_library_alloc_src_boxed_rs_977_0()
}
