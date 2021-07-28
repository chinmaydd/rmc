#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_145_0() {
        use std::sync::Arc;

        let my_arc = Arc::new(());
        Arc::downgrade(&my_arc);
    }
    _doctest_main_library_alloc_src_sync_rs_145_0()
}
