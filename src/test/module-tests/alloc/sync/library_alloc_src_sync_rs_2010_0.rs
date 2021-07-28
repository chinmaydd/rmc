#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2010_0() {
        use std::sync::Weak;

        let empty: Weak<i64> = Default::default();
        assert!(empty.upgrade().is_none());
    }
    _doctest_main_library_alloc_src_sync_rs_2010_0()
}
