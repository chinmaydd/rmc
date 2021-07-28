#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_477_0() {
        use std::alloc::System;

        let s = [10, 40, 30];
        let x = s.to_vec_in(System);
        // Here, `s` and `x` can be modified independently.
    }
    _doctest_main_library_alloc_src_slice_rs_477_0()
}
