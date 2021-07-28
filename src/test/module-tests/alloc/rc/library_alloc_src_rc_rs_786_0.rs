#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_786_0() {
        use std::rc::Rc;

        let x = Rc::new("hello".to_owned());
        let x_ptr = Rc::into_raw(x);
        assert_eq!(unsafe { &*x_ptr }, "hello");
    }
    _doctest_main_library_alloc_src_rc_rs_786_0()
}
