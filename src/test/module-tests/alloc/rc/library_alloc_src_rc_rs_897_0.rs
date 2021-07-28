#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_897_0() {
        use std::rc::Rc;

        let five = Rc::new(5);
        let _weak_five = Rc::downgrade(&five);

        assert_eq!(1, Rc::weak_count(&five));
    }
    _doctest_main_library_alloc_src_rc_rs_897_0()
}
