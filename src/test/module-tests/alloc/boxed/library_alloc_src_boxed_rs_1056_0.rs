#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_1056_0() {
        let x = Box::new(5);
        let y = x.clone();

        // The value is the same
        assert_eq!(x, y);

        // But they are unique objects
        assert_ne!(&*x as *const i32, &*y as *const i32);
    }
    _doctest_main_library_alloc_src_boxed_rs_1056_0()
}
