#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2411_0() {
        use std::sync::Arc;
        let unique: Vec<i32> = vec![1, 2, 3];
        let shared: Arc<[i32]> = Arc::from(unique);
        assert_eq!(&[1, 2, 3], &shared[..]);
    }
    _doctest_main_library_alloc_src_sync_rs_2411_0()
}
