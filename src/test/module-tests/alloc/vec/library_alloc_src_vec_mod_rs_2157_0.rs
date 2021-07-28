#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2157_0() {
        let mut vec = vec![0, 1, 2, 3, 4];

        vec.extend_from_within(2..);
        assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4]);

        vec.extend_from_within(..2);
        assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1]);

        vec.extend_from_within(4..8);
        assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1, 4, 2, 3, 4]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2157_0()
}
