#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_878_0() {
        use std::rc::Rc;

        let five = Rc::new(5);

        let weak_five = Rc::downgrade(&five);
    }
    _doctest_main_library_alloc_src_rc_rs_878_0()
}
