#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_906_0() {
        use std::alloc::{Allocator, Layout, System};
        use std::ptr::{self, NonNull};

        let x = Box::new_in(String::from("Hello"), System);
        let (ptr, alloc) = Box::into_raw_with_allocator(x);
        unsafe {
            ptr::drop_in_place(ptr);
            let non_null = NonNull::new_unchecked(ptr);
            alloc.deallocate(non_null.cast(), Layout::new::<String>());
        }
    }
    _doctest_main_library_alloc_src_boxed_rs_906_0()
}
