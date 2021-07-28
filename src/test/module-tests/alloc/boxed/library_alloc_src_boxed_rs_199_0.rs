#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_199_0() {
        let mut five = Box::<u32>::new_uninit();

        let five = unsafe {
            // Deferred initialization:
            five.as_mut_ptr().write(5);

            five.assume_init()
        };

        assert_eq!(*five, 5)
    }
    _doctest_main_library_alloc_src_boxed_rs_199_0()
}
