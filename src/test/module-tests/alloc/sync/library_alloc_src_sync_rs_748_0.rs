#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_748_0() {
        use std::sync::Arc;

        let mut values = Arc::<[u32]>::new_uninit_slice(3);

        let values = unsafe {
            // Deferred initialization:
            Arc::get_mut_unchecked(&mut values)[0].as_mut_ptr().write(1);
            Arc::get_mut_unchecked(&mut values)[1].as_mut_ptr().write(2);
            Arc::get_mut_unchecked(&mut values)[2].as_mut_ptr().write(3);

            values.assume_init()
        };

        assert_eq!(*values, [1, 2, 3])
    }
    _doctest_main_library_alloc_src_sync_rs_748_0()
}
