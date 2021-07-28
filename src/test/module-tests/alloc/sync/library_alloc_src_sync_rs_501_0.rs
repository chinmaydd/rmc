#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_501_0() -> Result<(), impl core::fmt::Debug> {
        use std::sync::Arc;

        let five = Arc::try_new(5)?;
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_sync_rs_501_0().unwrap()
}
