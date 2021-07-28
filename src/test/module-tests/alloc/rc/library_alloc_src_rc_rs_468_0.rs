#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_468_0() {
        use std::rc::Rc;

        let zero = Rc::<u32>::new_zeroed();
        let zero = unsafe { zero.assume_init() };

        assert_eq!(*zero, 0)
    }
    _doctest_main_library_alloc_src_rc_rs_468_0()
}
