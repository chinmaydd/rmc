#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, slice_ptr_get)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_807_0() -> Result<(), impl core::fmt::Debug> {
        use std::alloc::{Allocator, Layout, System};

        unsafe {
            let ptr = System.allocate(Layout::new::<i32>())?.as_mut_ptr() as *mut i32;
            // In general .write is required to avoid attempting to destruct
            // the (uninitialized) previous contents of `ptr`, though for this
            // simple example `*ptr = 5` would have worked as well.
            ptr.write(5);
            let x = Box::from_raw_in(ptr, System);
        }
        Ok::<(), std::alloc::AllocError>(())
    }
    _doctest_main_library_alloc_src_boxed_rs_807_0().unwrap()
}
