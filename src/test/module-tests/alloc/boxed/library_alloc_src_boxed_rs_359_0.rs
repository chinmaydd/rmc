#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_359_0() -> Result<(), impl core::fmt::Debug> {
        use std::alloc::System;

        let five = Box::try_new_in(5, System)?;
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_boxed_rs_359_0().unwrap()
}
