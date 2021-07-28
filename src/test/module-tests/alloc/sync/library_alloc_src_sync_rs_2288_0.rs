#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2288_0() {
        use std::sync::Arc;

        let x: Arc<i32> = Default::default();
        assert_eq!(*x, 0);
    }
    _doctest_main_library_alloc_src_sync_rs_2288_0()
}
