#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_alloc_rs_140_0() {
        use std::alloc::{alloc_zeroed, dealloc, Layout};

        unsafe {
            let layout = Layout::new::<u16>();
            let ptr = alloc_zeroed(layout);

            assert_eq!(*(ptr as *mut u16), 0);

            dealloc(ptr, layout);
        }
    }
    _doctest_main_library_alloc_src_alloc_rs_140_0()
}
