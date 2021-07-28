#![allow(unused_variables)]
#![deny(warnings)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_635_0() {
        use std::rc::Rc;

        let mut values = Rc::<[u32]>::new_uninit_slice(3);

        let values = unsafe {
            // Deferred initialization:
            Rc::get_mut_unchecked(&mut values)[0].as_mut_ptr().write(1);
            Rc::get_mut_unchecked(&mut values)[1].as_mut_ptr().write(2);
            Rc::get_mut_unchecked(&mut values)[2].as_mut_ptr().write(3);

            values.assume_init()
        };

        assert_eq!(*values, [1, 2, 3])
    }
    _doctest_main_library_alloc_src_rc_rs_635_0()
}
