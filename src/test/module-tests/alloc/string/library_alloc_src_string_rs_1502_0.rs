#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1502_0() {
        let mut s = String::from("hello");

        unsafe {
            let vec = s.as_mut_vec();
            assert_eq!(&[104, 101, 108, 108, 111][..], &vec[..]);

            vec.reverse();
        }
        assert_eq!(s, "olleh");
    }
    _doctest_main_library_alloc_src_string_rs_1502_0()
}
