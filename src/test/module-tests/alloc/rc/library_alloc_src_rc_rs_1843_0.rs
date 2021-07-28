#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1843_0() {
        use std::rc::Rc;
        let original: Box<Vec<i32>> = Box::new(vec![1, 2, 3]);
        let shared: Rc<Vec<i32>> = Rc::from(original);
        assert_eq!(vec![1, 2, 3], *shared);
    }
    _doctest_main_library_alloc_src_rc_rs_1843_0()
}
