#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_953_0() {
        use std::sync::Arc;

        let five = Arc::new(5);
        let _also_five = Arc::clone(&five);

        // This assertion is deterministic because we haven't shared
        // the `Arc` between threads.
        assert_eq!(2, Arc::strong_count(&five));
    }
    _doctest_main_library_alloc_src_sync_rs_953_0()
}
