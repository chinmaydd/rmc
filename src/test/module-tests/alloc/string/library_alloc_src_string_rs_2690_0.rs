#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_2690_0() {
        let s1 = String::from("hello world");
        let v1 = Vec::from(s1);

        for b in v1 {
            println!("{}", b);
        }
    }
    _doctest_main_library_alloc_src_string_rs_2690_0()
}
