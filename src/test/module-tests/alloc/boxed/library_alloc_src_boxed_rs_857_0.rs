#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_857_0() {
        use std::alloc::{dealloc, Layout};
        use std::ptr;

        let x = Box::new(String::from("Hello"));
        let p = Box::into_raw(x);
        unsafe {
            ptr::drop_in_place(p);
            dealloc(p as *mut u8, Layout::new::<String>());
        }
    }
    _doctest_main_library_alloc_src_boxed_rs_857_0()
}
