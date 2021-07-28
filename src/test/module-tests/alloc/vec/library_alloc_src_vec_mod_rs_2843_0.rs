#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2843_0() {
        use std::borrow::Cow;
        let o: Cow<[i32]> = Cow::Owned(vec![1, 2, 3]);
        let b: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
        assert_eq!(Vec::from(o), Vec::from(b));
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2843_0()
}
