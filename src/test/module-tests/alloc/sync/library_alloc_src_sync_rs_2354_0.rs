#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2354_0() {
        use std::sync::Arc;
        let shared: Arc<str> = Arc::from("eggplant");
        assert_eq!("eggplant", &shared[..]);
    }
    _doctest_main_library_alloc_src_sync_rs_2354_0()
}
