#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1938_0() {
        use std::rc::Rc;
        let evens: Rc<[u8]> = (0..10).collect(); // Just a single allocation happens here.
        assert_eq!(&*evens, &*(0..10).collect::<Vec<_>>());
    }
    _doctest_main_library_alloc_src_rc_rs_1938_0()
}
