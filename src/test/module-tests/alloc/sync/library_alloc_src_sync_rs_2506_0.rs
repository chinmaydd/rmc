#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2506_0() {
        use std::sync::Arc;
        let evens: Arc<[u8]> = (0..10).collect(); // Just a single allocation happens here.
        assert_eq!(&*evens, &*(0..10).collect::<Vec<_>>());
    }
    _doctest_main_library_alloc_src_sync_rs_2506_0()
}
