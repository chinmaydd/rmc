#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1744_0() {
        let mut v = vec![1, 2, 3];
        let u: Vec<_> = v.drain(1..).collect();
        assert_eq!(v, &[1]);
        assert_eq!(u, &[2, 3]);

        // A full range clears the vector
        v.drain(..);
        assert_eq!(v, &[]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1744_0()
}
