#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1547_0() {
        let mut v = String::new();
        assert!(v.is_empty());

        v.push('a');
        assert!(!v.is_empty());
    }
    _doctest_main_library_alloc_src_string_rs_1547_0()
}
