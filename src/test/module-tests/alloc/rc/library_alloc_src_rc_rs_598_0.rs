#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_598_0() {
        use std::rc::Rc;

        let x = Rc::new(3);
        assert_eq!(Rc::try_unwrap(x), Ok(3));

        let x = Rc::new(4);
        let _y = Rc::clone(&x);
        assert_eq!(*Rc::try_unwrap(x).unwrap_err(), 4);
    }
    _doctest_main_library_alloc_src_rc_rs_598_0()
}
