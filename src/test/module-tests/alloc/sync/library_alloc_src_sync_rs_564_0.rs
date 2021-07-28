#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit, allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_564_0() -> Result<(), impl core::fmt::Debug> {
        use std::sync::Arc;

        let zero = Arc::<u32>::try_new_zeroed()?;
        let zero = unsafe { zero.assume_init() };

        assert_eq!(*zero, 0);
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_sync_rs_564_0().unwrap()
}
