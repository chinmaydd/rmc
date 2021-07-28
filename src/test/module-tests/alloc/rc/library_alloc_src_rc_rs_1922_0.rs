#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1922_0() {
        use std::rc::Rc;
        let evens: Rc<[u8]> = (0..10)
            .filter(|&x| x % 2 == 0)
            .collect::<Vec<_>>() // The first set of allocations happens here.
            .into(); // A second allocation for `Rc<[T]>` happens here.
        assert_eq!(&*evens, &[0, 2, 4, 6, 8]);
    }
    _doctest_main_library_alloc_src_rc_rs_1922_0()
}
