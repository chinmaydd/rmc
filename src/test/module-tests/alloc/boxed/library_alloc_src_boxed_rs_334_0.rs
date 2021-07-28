#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_334_0() {
        use std::alloc::System;

        let five = Box::new_in(5, System);
    }
    _doctest_main_library_alloc_src_boxed_rs_334_0()
}
