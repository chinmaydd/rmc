#![allow(unused_variables)]
#![deny(warnings)]
#![feature(get_mut_unchecked)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1482_0() {
        use std::sync::Arc;

        let mut x = Arc::new(String::new());
        unsafe { Arc::get_mut_unchecked(&mut x).push_str("foo") }
        assert_eq!(*x, "foo");
    }
    _doctest_main_library_alloc_src_sync_rs_1482_0()
}
