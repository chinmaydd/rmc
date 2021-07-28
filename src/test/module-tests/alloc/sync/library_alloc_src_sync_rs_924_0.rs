#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_924_0() {
        use std::sync::Arc;

        let five = Arc::new(5);
        let _weak_five = Arc::downgrade(&five);

        // This assertion is deterministic because we haven't shared
        // the `Arc` or `Weak` between threads.
        assert_eq!(1, Arc::weak_count(&five));
    }
    _doctest_main_library_alloc_src_sync_rs_924_0()
}
