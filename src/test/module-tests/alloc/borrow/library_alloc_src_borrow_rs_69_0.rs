#![allow(unused_variables)]
#![deny(warnings)]
#![feature(toowned_clone_into)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_borrow_rs_69_0() {
        let mut s: String = String::new();
        "hello".clone_into(&mut s);

        let mut v: Vec<i32> = Vec::new();
        [1, 2][..].clone_into(&mut v);
    }
    _doctest_main_library_alloc_src_borrow_rs_69_0()
}
