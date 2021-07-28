#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_146_0() {
        println!("Hello {:^15}!", format!("{:?}", Some("hi"))); // => "Hello   Some("hi")   !"
    }
    _doctest_main_library_alloc_src_fmt_rs_146_0()
}
