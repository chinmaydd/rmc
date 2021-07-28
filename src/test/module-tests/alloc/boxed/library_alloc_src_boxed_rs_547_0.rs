#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_547_0() {
        let mut values = Box::<[u32]>::new_uninit_slice(3);

        let values = unsafe {
            // Deferred initialization:
            values[0].as_mut_ptr().write(1);
            values[1].as_mut_ptr().write(2);
            values[2].as_mut_ptr().write(3);

            values.assume_init()
        };

        assert_eq!(*values, [1, 2, 3])
    }
    _doctest_main_library_alloc_src_boxed_rs_547_0()
}
