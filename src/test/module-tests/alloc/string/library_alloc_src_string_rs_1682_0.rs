#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1682_0() {
        let mut s = String::from("α is alpha, β is beta");
        let beta_offset = s.find('β').unwrap_or(s.len());

        // Replace the range up until the β from the string
        s.replace_range(..beta_offset, "Α is capital alpha; ");
        assert_eq!(s, "Α is capital alpha; β is beta");
    }
    _doctest_main_library_alloc_src_string_rs_1682_0()
}
