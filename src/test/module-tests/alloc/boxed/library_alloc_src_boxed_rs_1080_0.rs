#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_1080_0() {
        let x = Box::new(5);
        let mut y = Box::new(10);
        let yp: *const i32 = &*y;

        y.clone_from(&x);

        // The value is the same
        assert_eq!(x, y);

        // And no allocation occurred
        assert_eq!(yp, &*y);
    }
    _doctest_main_library_alloc_src_boxed_rs_1080_0()
}
