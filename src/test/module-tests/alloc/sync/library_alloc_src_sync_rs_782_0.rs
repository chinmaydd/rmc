#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_782_0() {
        use std::sync::Arc;

        let x = Arc::new("hello".to_owned());
        let x_ptr = Arc::into_raw(x);
        assert_eq!(unsafe { &*x_ptr }, "hello");
    }
    _doctest_main_library_alloc_src_sync_rs_782_0()
}
