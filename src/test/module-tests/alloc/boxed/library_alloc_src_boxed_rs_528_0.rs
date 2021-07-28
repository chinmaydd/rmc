#![allow(unused_variables)]
#![deny(warnings)]
#![feature(box_into_inner)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_528_0() {
        let c = Box::new(5);

        assert_eq!(Box::into_inner(c), 5);
    }
    _doctest_main_library_alloc_src_boxed_rs_528_0()
}
