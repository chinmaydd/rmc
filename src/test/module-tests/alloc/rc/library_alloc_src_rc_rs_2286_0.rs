#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2286_0() {
        use std::rc::Rc;

        let first_rc = Rc::new(5);
        let first = Rc::downgrade(&first_rc);
        let second = Rc::downgrade(&first_rc);

        assert!(first.ptr_eq(&second));

        let third_rc = Rc::new(5);
        let third = Rc::downgrade(&third_rc);

        assert!(!first.ptr_eq(&third));
    }
    _doctest_main_library_alloc_src_rc_rs_2286_0()
}
