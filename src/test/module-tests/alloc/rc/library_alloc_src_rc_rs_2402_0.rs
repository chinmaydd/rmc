#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2402_0() {
        use std::rc::Weak;

        let empty: Weak<i64> = Default::default();
        assert!(empty.upgrade().is_none());
    }
    _doctest_main_library_alloc_src_rc_rs_2402_0()
}
