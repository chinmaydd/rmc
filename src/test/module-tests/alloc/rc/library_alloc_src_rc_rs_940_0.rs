#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_940_0() {
        use std::rc::Rc;

        let five = Rc::new(5);

        unsafe {
            let ptr = Rc::into_raw(five);
            Rc::increment_strong_count(ptr);

            let five = Rc::from_raw(ptr);
            assert_eq!(2, Rc::strong_count(&five));
        }
    }
    _doctest_main_library_alloc_src_rc_rs_940_0()
}
