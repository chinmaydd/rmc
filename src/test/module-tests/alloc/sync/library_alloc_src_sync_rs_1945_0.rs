#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1945_0() {
        use std::sync::{Arc, Weak};

        let first = Weak::new();
        let second = Weak::new();
        assert!(first.ptr_eq(&second));

        let third_rc = Arc::new(());
        let third = Arc::downgrade(&third_rc);
        assert!(!first.ptr_eq(&third));
    }
    _doctest_main_library_alloc_src_sync_rs_1945_0()
}
