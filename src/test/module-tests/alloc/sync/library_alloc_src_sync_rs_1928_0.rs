#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1928_0() {
        use std::sync::Arc;

        let first_rc = Arc::new(5);
        let first = Arc::downgrade(&first_rc);
        let second = Arc::downgrade(&first_rc);

        assert!(first.ptr_eq(&second));

        let third_rc = Arc::new(5);
        let third = Arc::downgrade(&third_rc);

        assert!(!first.ptr_eq(&third));
    }
    _doctest_main_library_alloc_src_sync_rs_1928_0()
}
