#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_40_0() {
        use std::rc::Rc;

        let my_rc = Rc::new(());
        Rc::downgrade(&my_rc);
    }
    _doctest_main_library_alloc_src_rc_rs_40_0()
}
