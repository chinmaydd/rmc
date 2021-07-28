#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1017_0() {
        use std::rc::Rc;

        let mut x = Rc::new(3);
        *Rc::get_mut(&mut x).unwrap() = 4;
        assert_eq!(*x, 4);

        let _y = Rc::clone(&x);
        assert!(Rc::get_mut(&mut x).is_none());
    }
    _doctest_main_library_alloc_src_rc_rs_1017_0()
}
