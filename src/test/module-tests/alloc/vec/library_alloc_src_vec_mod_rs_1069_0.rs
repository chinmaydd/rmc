#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1069_0() {
        use std::io::{self, Write};
        let buffer = vec![1, 2, 3, 5, 8];
        io::sink().write(buffer.as_slice()).unwrap();
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1069_0()
}
