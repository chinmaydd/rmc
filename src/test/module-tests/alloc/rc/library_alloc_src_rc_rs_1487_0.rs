#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1487_0() {
        use std::rc::Rc;

        let x: Rc<i32> = Default::default();
        assert_eq!(*x, 0);
    }
    _doctest_main_library_alloc_src_rc_rs_1487_0()
}
