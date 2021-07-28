#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_668_0() {
        use std::rc::Rc;

        let values = Rc::<[u32]>::new_zeroed_slice(3);
        let values = unsafe { values.assume_init() };

        assert_eq!(*values, [0, 0, 0])
    }
    _doctest_main_library_alloc_src_rc_rs_668_0()
}
