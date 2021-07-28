#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1422_0() {
        let mut s = String::with_capacity(3);

        s.insert(0, 'f');
        s.insert(1, 'o');
        s.insert(2, 'o');

        assert_eq!("foo", s);
    }
    _doctest_main_library_alloc_src_string_rs_1422_0()
}
