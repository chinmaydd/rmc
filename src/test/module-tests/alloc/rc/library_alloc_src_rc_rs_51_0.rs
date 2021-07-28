#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_51_0() {
        use std::rc::Rc;

        let rc = Rc::new(());
        // Method-call syntax
        let rc2 = rc.clone();
        // Fully qualified syntax
        let rc3 = Rc::clone(&rc);
    }
    _doctest_main_library_alloc_src_rc_rs_51_0()
}
