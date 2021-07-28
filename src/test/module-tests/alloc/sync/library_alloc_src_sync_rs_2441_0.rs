#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2441_0() {
        use std::borrow::Cow;
        use std::sync::Arc;
        let cow: Cow<str> = Cow::Borrowed("eggplant");
        let shared: Arc<str> = Arc::from(cow);
        assert_eq!("eggplant", &shared[..]);
    }
    _doctest_main_library_alloc_src_sync_rs_2441_0()
}
