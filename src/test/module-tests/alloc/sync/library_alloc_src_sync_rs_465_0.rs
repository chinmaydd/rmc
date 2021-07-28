#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_465_0() {
        use std::sync::Arc;

        let zero = Arc::<u32>::new_zeroed();
        let zero = unsafe { zero.assume_init() };

        assert_eq!(*zero, 0)
    }
    _doctest_main_library_alloc_src_sync_rs_465_0()
}
