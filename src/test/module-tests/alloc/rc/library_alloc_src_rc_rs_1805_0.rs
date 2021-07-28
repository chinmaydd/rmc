#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1805_0() {
        use std::rc::Rc;
        let original: String = "statue".to_owned();
        let shared: Rc<str> = Rc::from(original);
        assert_eq!("statue", &shared[..]);
    }
    _doctest_main_library_alloc_src_rc_rs_1805_0()
}
