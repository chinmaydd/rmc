#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1632_0() {
        let mut s = String::from("α is alpha, β is beta");
        let beta_offset = s.find('β').unwrap_or(s.len());

        // Remove the range up until the β from the string
        let t: String = s.drain(..beta_offset).collect();
        assert_eq!(t, "α is alpha, ");
        assert_eq!(s, "β is beta");

        // A full range clears the string
        s.drain(..);
        assert_eq!(s, "");
    }
    _doctest_main_library_alloc_src_string_rs_1632_0()
}
