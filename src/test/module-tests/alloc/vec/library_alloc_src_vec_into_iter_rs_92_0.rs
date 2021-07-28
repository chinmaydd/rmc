#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_into_iter_rs_92_0() {
        let mut into_iter = Vec::<u8>::with_capacity(10).into_iter();
        (&mut into_iter).for_each(core::mem::drop);
        unsafe {
            core::ptr::write(&mut into_iter, Vec::new().into_iter());
        }
    }
    _doctest_main_library_alloc_src_vec_into_iter_rs_92_0()
}
