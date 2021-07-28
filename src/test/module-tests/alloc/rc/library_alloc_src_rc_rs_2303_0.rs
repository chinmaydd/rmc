#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2303_0() {
        use std::rc::{Rc, Weak};

        let first = Weak::new();
        let second = Weak::new();
        assert!(first.ptr_eq(&second));

        let third_rc = Rc::new(());
        let third = Rc::downgrade(&third_rc);
        assert!(!first.ptr_eq(&third));
    }
    _doctest_main_library_alloc_src_rc_rs_2303_0()
}
