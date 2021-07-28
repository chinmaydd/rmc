#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_797_0() {
        use std::alloc::System;

        let x = Box::new_in(5, System);
        let (ptr, alloc) = Box::into_raw_with_allocator(x);
        let x = unsafe { Box::from_raw_in(ptr, alloc) };
    }
    _doctest_main_library_alloc_src_boxed_rs_797_0()
}
