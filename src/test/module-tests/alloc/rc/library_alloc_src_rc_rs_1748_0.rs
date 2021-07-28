#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1748_0() {
        use std::rc::Rc;
        let x = 5;
        let rc = Rc::new(5);

        assert_eq!(Rc::from(x), rc);
    }
    _doctest_main_library_alloc_src_rc_rs_1748_0()
}
