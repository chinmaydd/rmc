#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2622_0() {
        use std::borrow::Cow;
        let s = "eggplant".to_string();
        let s2 = "eggplant".to_string();
        assert_eq!(Cow::from(s), Cow::<'static, str>::Owned(s2));
    }
    _doctest_main_library_alloc_src_string_rs_2622_0()
}
