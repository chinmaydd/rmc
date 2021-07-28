#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_415_0() -> Result<(), impl core::fmt::Debug> {
        use std::alloc::System;

        let mut five = Box::<u32, _>::try_new_uninit_in(System)?;

        let five = unsafe {
            // Deferred initialization:
            five.as_mut_ptr().write(5);

            five.assume_init()
        };

        assert_eq!(*five, 5);
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_boxed_rs_415_0().unwrap()
}
