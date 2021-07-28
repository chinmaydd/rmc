#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_alloc_rs_70_0() {
        use std::alloc::{alloc, dealloc, Layout};

        unsafe {
            let layout = Layout::new::<u16>();
            let ptr = alloc(layout);

            *(ptr as *mut u16) = 42;
            assert_eq!(*(ptr as *mut u16), 42);

            dealloc(ptr, layout);
        }
    }
    _doctest_main_library_alloc_src_alloc_rs_70_0()
}
