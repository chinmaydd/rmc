#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_632_0() {
        use std::alloc::System;

        let values = Box::<[u32], _>::new_zeroed_slice_in(3, System);
        let values = unsafe { values.assume_init() };

        assert_eq!(*values, [0, 0, 0])
    }
    _doctest_main_library_alloc_src_boxed_rs_632_0()
}
