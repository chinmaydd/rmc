#![allow(unused_variables)]
#![deny(warnings)]
// create a Box<str> which will be used to create a Box<[u8]>
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_1315_0() {
        let boxed: Box<str> = Box::from("hello");
        let boxed_str: Box<[u8]> = Box::from(boxed);

        // create a &[u8] which will be used to create a Box<[u8]>
        let slice: &[u8] = &[104, 101, 108, 108, 111];
        let boxed_slice = Box::from(slice);

        assert_eq!(boxed_slice, boxed_str);
    }
    _doctest_main_library_alloc_src_boxed_rs_1315_0()
}
