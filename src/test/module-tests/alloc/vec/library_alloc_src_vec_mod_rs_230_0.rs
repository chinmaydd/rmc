#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_230_0() {
        let v = vec![0, 2, 4, 6];
        println!("{}", v[6]); // it will panic!
    }
    _doctest_main_library_alloc_src_vec_mod_rs_230_0()
}
