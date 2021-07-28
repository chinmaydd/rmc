#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1873_0() {
        use std::borrow::Cow;
        use std::rc::Rc;
        let cow: Cow<str> = Cow::Borrowed("eggplant");
        let shared: Rc<str> = Rc::from(cow);
        assert_eq!("eggplant", &shared[..]);
    }
    _doctest_main_library_alloc_src_rc_rs_1873_0()
}
