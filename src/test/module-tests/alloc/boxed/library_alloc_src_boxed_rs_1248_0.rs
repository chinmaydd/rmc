#![allow(unused_variables)]
#![deny(warnings)]
// create a &[u8] which will be used to create a Box<[u8]>
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_1248_0() {
        let slice: &[u8] = &[104, 101, 108, 108, 111];
        let boxed_slice: Box<[u8]> = Box::from(slice);

        println!("{:?}", boxed_slice);
    }
    _doctest_main_library_alloc_src_boxed_rs_1248_0()
}
