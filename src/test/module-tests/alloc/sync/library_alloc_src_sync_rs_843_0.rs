#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_843_0() {
        use std::sync::Arc;

        let x = Arc::new("hello".to_owned());
        let x_ptr = Arc::into_raw(x);

        unsafe {
            // Convert back to an `Arc` to prevent leak.
            let x = Arc::from_raw(x_ptr);
            assert_eq!(&*x, "hello");

            // Further calls to `Arc::from_raw(x_ptr)` would be memory-unsafe.
        }

        // The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
    }
    _doctest_main_library_alloc_src_sync_rs_843_0()
}
