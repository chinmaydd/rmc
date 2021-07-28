#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_156_0() {
        use std::sync::Arc;

        let arc = Arc::new(());
        // Method-call syntax
        let arc2 = arc.clone();
        // Fully qualified syntax
        let arc3 = Arc::clone(&arc);
    }
    _doctest_main_library_alloc_src_sync_rs_156_0()
}
