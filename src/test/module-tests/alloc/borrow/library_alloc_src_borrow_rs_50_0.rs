#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_borrow_rs_50_0() {
        let s: &str = "a";
        let ss: String = s.to_owned();

        let v: &[i32] = &[1, 2];
        let vv: Vec<i32> = v.to_owned();
    }
    _doctest_main_library_alloc_src_borrow_rs_50_0()
}
