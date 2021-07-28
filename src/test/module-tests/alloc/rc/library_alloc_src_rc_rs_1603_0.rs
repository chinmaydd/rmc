#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1603_0() {
        use std::cmp::Ordering;
        use std::rc::Rc;

        let five = Rc::new(5);

        assert_eq!(Some(Ordering::Less), five.partial_cmp(&Rc::new(6)));
    }
    _doctest_main_library_alloc_src_rc_rs_1603_0()
}
