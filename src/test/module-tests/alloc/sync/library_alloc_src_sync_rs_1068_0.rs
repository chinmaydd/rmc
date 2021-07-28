#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1068_0() {
        use std::sync::Arc;

        let five = Arc::new(5);
        let same_five = Arc::clone(&five);
        let other_five = Arc::new(5);

        assert!(Arc::ptr_eq(&five, &same_five));
        assert!(!Arc::ptr_eq(&five, &other_five));
    }
    _doctest_main_library_alloc_src_sync_rs_1068_0()
}
