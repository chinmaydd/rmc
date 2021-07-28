#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2468_0() {
        let v = vec!["a".to_string(), "b".to_string()];
        for s in v.into_iter() {
            // s has type String, not &String
            println!("{}", s);
        }
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2468_0()
}
