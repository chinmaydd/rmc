#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_str_rs_351_0() {
        let sigma = "Σ";

        assert_eq!("σ", sigma.to_lowercase());

        // but at the end of a word, it's ς, not σ:
        let odysseus = "ὈΔΥΣΣΕΎΣ";

        assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());
    }
    _doctest_main_library_alloc_src_str_rs_351_0()
}
