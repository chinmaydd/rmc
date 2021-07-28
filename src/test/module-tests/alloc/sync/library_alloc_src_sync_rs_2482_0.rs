#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2482_0() {
        use std::sync::Arc;
        let evens: Arc<[u8]> = (0..10).filter(|&x| x % 2 == 0).collect();
        assert_eq!(&*evens, &[0, 2, 4, 6, 8]);
    }
    _doctest_main_library_alloc_src_sync_rs_2482_0()
}
