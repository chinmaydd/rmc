#![allow(unused_variables)]
#![deny(warnings)]
#![feature(get_mut_unchecked)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1049_0() {
        use std::rc::Rc;

        let mut x = Rc::new(String::new());
        unsafe { Rc::get_mut_unchecked(&mut x).push_str("foo") }
        assert_eq!(*x, "foo");
    }
    _doctest_main_library_alloc_src_rc_rs_1049_0()
}
