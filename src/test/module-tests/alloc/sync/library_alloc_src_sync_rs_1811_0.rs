#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1811_0() {
        use std::sync::Arc;

        let five = Arc::new(5);

        let weak_five = Arc::downgrade(&five);

        let strong_five: Option<Arc<_>> = weak_five.upgrade();
        assert!(strong_five.is_some());

        // Destroy all strong pointers.
        drop(strong_five);
        drop(five);

        assert!(weak_five.upgrade().is_none());
    }
    _doctest_main_library_alloc_src_sync_rs_1811_0()
}
