#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1284_0() {
        use std::sync::Arc;

        let five = Arc::new(5);

        let _ = Arc::clone(&five);
    }
    _doctest_main_library_alloc_src_sync_rs_1284_0()
}
