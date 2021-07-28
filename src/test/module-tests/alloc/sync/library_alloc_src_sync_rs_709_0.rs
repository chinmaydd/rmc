#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_709_0() {
        use std::sync::Arc;

        let mut five = Arc::<u32>::new_uninit();

        let five = unsafe {
            // Deferred initialization:
            Arc::get_mut_unchecked(&mut five).as_mut_ptr().write(5);

            five.assume_init()
        };

        assert_eq!(*five, 5)
    }
    _doctest_main_library_alloc_src_sync_rs_709_0()
}
