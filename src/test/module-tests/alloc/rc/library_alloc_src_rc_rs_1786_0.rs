#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1786_0() {
        use std::rc::Rc;
        let shared: Rc<str> = Rc::from("statue");
        assert_eq!("statue", &shared[..]);
    }
    _doctest_main_library_alloc_src_rc_rs_1786_0()
}
