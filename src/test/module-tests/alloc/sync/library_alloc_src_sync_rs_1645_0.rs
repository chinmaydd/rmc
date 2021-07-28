#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1645_0() {
        use std::sync::Weak;

        let empty: Weak<i64> = Weak::new();
        assert!(empty.upgrade().is_none());
    }
    _doctest_main_library_alloc_src_sync_rs_1645_0()
}
