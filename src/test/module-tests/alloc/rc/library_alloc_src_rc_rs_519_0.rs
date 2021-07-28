#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, new_uninit)]
#![feature(get_mut_unchecked)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_519_0() -> Result<(), impl core::fmt::Debug> {
        use std::rc::Rc;

        let mut five = Rc::<u32>::try_new_uninit()?;

        let five = unsafe {
            // Deferred initialization:
            Rc::get_mut_unchecked(&mut five).as_mut_ptr().write(5);

            five.assume_init()
        };

        assert_eq!(*five, 5);
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_rc_rs_519_0().unwrap()
}
