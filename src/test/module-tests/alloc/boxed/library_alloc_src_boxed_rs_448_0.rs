#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_448_0() {
        use std::alloc::System;

        let zero = Box::<u32, _>::new_zeroed_in(System);
        let zero = unsafe { zero.assume_init() };

        assert_eq!(*zero, 0)
    }
    _doctest_main_library_alloc_src_boxed_rs_448_0()
}
