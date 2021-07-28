#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1767_0() {
        use std::rc::Rc;
        let original: &[i32] = &[1, 2, 3];
        let shared: Rc<[i32]> = Rc::from(original);
        assert_eq!(&[1, 2, 3], &shared[..]);
    }
    _doctest_main_library_alloc_src_rc_rs_1767_0()
}
