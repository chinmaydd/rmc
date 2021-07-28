#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1241_0() {
        let mut vec = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        // SAFETY:
        // 1. `old_len..0` is empty so no elements need to be initialized.
        // 2. `0 <= capacity` always holds whatever `capacity` is.
        unsafe {
            vec.set_len(0);
        }
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1241_0()
}
