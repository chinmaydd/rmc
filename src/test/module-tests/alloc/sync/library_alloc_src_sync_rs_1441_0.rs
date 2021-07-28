#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1441_0() {
        use std::sync::Arc;

        let mut x = Arc::new(3);
        *Arc::get_mut(&mut x).unwrap() = 4;
        assert_eq!(*x, 4);

        let _y = Arc::clone(&x);
        assert!(Arc::get_mut(&mut x).is_none());
    }
    _doctest_main_library_alloc_src_sync_rs_1441_0()
}
