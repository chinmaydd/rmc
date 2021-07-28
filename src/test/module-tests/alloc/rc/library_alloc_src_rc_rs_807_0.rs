#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_807_0() {
        use std::rc::Rc;

        let x = Rc::new("hello".to_owned());
        let y = Rc::clone(&x);
        let x_ptr = Rc::as_ptr(&x);
        assert_eq!(x_ptr, Rc::as_ptr(&y));
        assert_eq!(unsafe { &*x_ptr }, "hello");
    }
    _doctest_main_library_alloc_src_rc_rs_807_0()
}
