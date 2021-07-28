#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_2157_0() {
        use std::cmp::Ordering;
        use std::sync::Arc;

        let five = Arc::new(5);

        assert_eq!(Some(Ordering::Less), five.partial_cmp(&Arc::new(6)));
    }
    _doctest_main_library_alloc_src_sync_rs_2157_0()
}
