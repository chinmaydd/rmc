#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2369_0() {
        use std::rc::{Rc, Weak};

        let weak_five = Rc::downgrade(&Rc::new(5));

        let _ = Weak::clone(&weak_five);
    }
    _doctest_main_library_alloc_src_rc_rs_2369_0()
}
