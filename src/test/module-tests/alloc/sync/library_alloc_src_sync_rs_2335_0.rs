#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2335_0() {
        use std::sync::Arc;
        let original: &[i32] = &[1, 2, 3];
        let shared: Arc<[i32]> = Arc::from(original);
        assert_eq!(&[1, 2, 3], &shared[..]);
    }
    _doctest_main_library_alloc_src_sync_rs_2335_0()
}
