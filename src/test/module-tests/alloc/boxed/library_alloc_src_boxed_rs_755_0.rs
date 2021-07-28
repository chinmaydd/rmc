#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_755_0() {
        use std::alloc::{alloc, Layout};

        unsafe {
            let ptr = alloc(Layout::new::<i32>()) as *mut i32;
            // In general .write is required to avoid attempting to destruct
            // the (uninitialized) previous contents of `ptr`, though for this
            // simple example `*ptr = 5` would have worked as well.
            ptr.write(5);
            let x = Box::from_raw(ptr);
        }
    }
    _doctest_main_library_alloc_src_boxed_rs_755_0()
}
