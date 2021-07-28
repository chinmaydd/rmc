#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1110_0() {
        let x = vec![1, 2, 4];
        let x_ptr = x.as_ptr();

        unsafe {
            for i in 0..x.len() {
                assert_eq!(*x_ptr.add(i), 1 << i);
            }
        }
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1110_0()
}
