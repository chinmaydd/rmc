#![allow(unused_variables)]
#![deny(warnings)]
#![feature(drain_filter)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2644_0() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15];

        let evens = numbers.drain_filter(|x| *x % 2 == 0).collect::<Vec<_>>();
        let odds = numbers;

        assert_eq!(evens, vec![2, 4, 6, 8, 14]);
        assert_eq!(odds, vec![1, 3, 5, 9, 11, 13, 15]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2644_0()
}
