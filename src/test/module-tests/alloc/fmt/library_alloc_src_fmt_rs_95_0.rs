#![allow(unused_variables)]
#![deny(warnings)]
// All of these print "Hello x    !"
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_95_0() {
        println!("Hello {:5}!", "x");
        println!("Hello {:1$}!", "x", 5);
        println!("Hello {1:0$}!", 5, "x");
        println!("Hello {:width$}!", "x", width = 5);
    }
    _doctest_main_library_alloc_src_fmt_rs_95_0()
}
