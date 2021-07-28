#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_457_0() {
        let s = [10, 40, 30];
        let x = s.to_vec();
        // Here, `s` and `x` can be modified independently.
    }
    _doctest_main_library_alloc_src_slice_rs_457_0()
}
