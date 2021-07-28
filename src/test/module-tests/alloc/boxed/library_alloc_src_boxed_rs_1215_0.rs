#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_1215_0() {
        let x = 5;
        let boxed = Box::new(5);

        assert_eq!(Box::from(x), boxed);
    }
    _doctest_main_library_alloc_src_boxed_rs_1215_0()
}
