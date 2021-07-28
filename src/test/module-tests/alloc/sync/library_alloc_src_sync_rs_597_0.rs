#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_597_0() {
        use std::sync::Arc;

        let x = Arc::new(3);
        assert_eq!(Arc::try_unwrap(x), Ok(3));

        let x = Arc::new(4);
        let _y = Arc::clone(&x);
        assert_eq!(*Arc::try_unwrap(x).unwrap_err(), 4);
    }
    _doctest_main_library_alloc_src_sync_rs_597_0()
}
