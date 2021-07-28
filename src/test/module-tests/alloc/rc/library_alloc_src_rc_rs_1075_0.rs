#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1075_0() {
        use std::rc::Rc;

        let five = Rc::new(5);
        let same_five = Rc::clone(&five);
        let other_five = Rc::new(5);

        assert!(Rc::ptr_eq(&five, &same_five));
        assert!(!Rc::ptr_eq(&five, &other_five));
    }
    _doctest_main_library_alloc_src_rc_rs_1075_0()
}
