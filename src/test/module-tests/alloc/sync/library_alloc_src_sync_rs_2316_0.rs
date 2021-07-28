#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2316_0() {
        use std::sync::Arc;
        let x = 5;
        let arc = Arc::new(5);

        assert_eq!(Arc::from(x), arc);
    }
    _doctest_main_library_alloc_src_sync_rs_2316_0()
}
