#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1824_0() {
        use std::rc::Rc;
        let original: Box<i32> = Box::new(1);
        let shared: Rc<i32> = Rc::from(original);
        assert_eq!(1, *shared);
    }
    _doctest_main_library_alloc_src_rc_rs_1824_0()
}
