#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_543_0() {
        use std::alloc::System;

        #[allow(unused_mut)]
        let mut vec: Vec<i32, _> = Vec::new_in(System);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_543_0()
}
