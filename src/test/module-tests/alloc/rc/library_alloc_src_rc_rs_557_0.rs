#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_557_0() -> Result<(), impl core::fmt::Debug> {
        use std::rc::Rc;

        let zero = Rc::<u32>::try_new_zeroed()?;
        let zero = unsafe { zero.assume_init() };

        assert_eq!(*zero, 0);
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_rc_rs_557_0().unwrap()
}
