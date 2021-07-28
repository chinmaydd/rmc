#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_482_0() -> Result<(), impl core::fmt::Debug> {
        use std::alloc::System;

        let zero = Box::<u32, _>::try_new_zeroed_in(System)?;
        let zero = unsafe { zero.assume_init() };

        assert_eq!(*zero, 0);
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_boxed_rs_482_0().unwrap()
}
