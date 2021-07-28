#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_258_0() {
        let mut s = String::with_capacity(25);

        println!("{}", s.capacity());

        for _ in 0..5 {
            s.push_str("hello");
            println!("{}", s.capacity());
        }
    }
    _doctest_main_library_alloc_src_string_rs_258_0()
}
