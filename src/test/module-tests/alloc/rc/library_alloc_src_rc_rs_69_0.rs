#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_69_0() {
        use std::rc::Rc;

        let foo = Rc::new(vec![1.0, 2.0, 3.0]);
        // The two syntaxes below are equivalent.
        let a = foo.clone();
        let b = Rc::clone(&foo);
        // a and b both point to the same memory location as foo.
    }
    _doctest_main_library_alloc_src_rc_rs_69_0()
}
