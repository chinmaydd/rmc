#![allow(unused_variables)]
#![deny(warnings)]
#![feature(build_hasher_simple_hash_one)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2405_0() {
        use std::hash::BuildHasher;

        let b = std::collections::hash_map::RandomState::new();
        let v: Vec<u8> = vec![0xa8, 0x3c, 0x09];
        let s: &[u8] = &[0xa8, 0x3c, 0x09];
        assert_eq!(b.hash_one(v), b.hash_one(s));
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2405_0()
}
