#![allow(unused_variables)]
#![deny(warnings)]
// Allocate vector big enough for 4 elements.
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1143_0() {
        let size = 4;
        let mut x: Vec<i32> = Vec::with_capacity(size);
        let x_ptr = x.as_mut_ptr();

        // Initialize elements via raw pointer writes, then set length.
        unsafe {
            for i in 0..size {
                *x_ptr.add(i) = i as i32;
            }
            x.set_len(size);
        }
        assert_eq!(&*x, &[0, 1, 2, 3]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1143_0()
}
