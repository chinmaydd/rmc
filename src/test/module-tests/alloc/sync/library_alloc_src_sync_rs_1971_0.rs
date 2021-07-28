#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1971_0() {
        use std::sync::{Arc, Weak};

        let weak_five = Arc::downgrade(&Arc::new(5));

        let _ = Weak::clone(&weak_five);
    }
    _doctest_main_library_alloc_src_sync_rs_1971_0()
}
