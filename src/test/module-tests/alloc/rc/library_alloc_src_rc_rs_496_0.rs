#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_496_0() -> Result<(), impl core::fmt::Debug> {
        use std::rc::Rc;

        let five = Rc::try_new(5);
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_rc_rs_496_0().unwrap()
}
