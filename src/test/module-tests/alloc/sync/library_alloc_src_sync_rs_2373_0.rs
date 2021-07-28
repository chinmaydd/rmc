#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2373_0() {
        use std::sync::Arc;
        let unique: String = "eggplant".to_owned();
        let shared: Arc<str> = Arc::from(unique);
        assert_eq!("eggplant", &shared[..]);
    }
    _doctest_main_library_alloc_src_sync_rs_2373_0()
}
