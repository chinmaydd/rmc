#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_57_0() {
        let mut scores = [7, 8, 9];
        for score in &mut scores[..] {
            *score += 1;
        }
    }
    _doctest_main_library_alloc_src_slice_rs_57_0()
}
