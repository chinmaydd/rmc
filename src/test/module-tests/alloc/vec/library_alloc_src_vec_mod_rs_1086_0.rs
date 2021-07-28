#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1086_0() {
        use std::io::{self, Read};
        let mut buffer = vec![0; 3];
        io::repeat(0b101).read_exact(buffer.as_mut_slice()).unwrap();
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1086_0()
}
