#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_915_0() {
        use std::rc::Rc;

        let five = Rc::new(5);
        let _also_five = Rc::clone(&five);

        assert_eq!(2, Rc::strong_count(&five));
    }
    _doctest_main_library_alloc_src_rc_rs_915_0()
}
