#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_261_0() -> Result<(), impl core::fmt::Debug> {
        let five = Box::try_new(5)?;
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_boxed_rs_261_0().unwrap()
}
