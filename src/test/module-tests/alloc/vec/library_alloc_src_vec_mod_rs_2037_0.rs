#![allow(unused_variables)]
#![deny(warnings)]
#![feature(vec_split_at_spare, maybe_uninit_extra)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2037_0() {
        let mut v = vec![1, 1, 2];

        // Reserve additional space big enough for 10 elements.
        v.reserve(10);

        let (init, uninit) = v.split_at_spare_mut();
        let sum = init.iter().copied().sum::<u32>();

        // Fill in the next 4 elements.
        uninit[0].write(sum);
        uninit[1].write(sum * 2);
        uninit[2].write(sum * 3);
        uninit[3].write(sum * 4);

        // Mark the 4 elements of the vector as being initialized.
        unsafe {
            let len = v.len();
            v.set_len(len + 4);
        }

        assert_eq!(&v, &[1, 1, 2, 4, 8, 12, 16]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2037_0()
}
