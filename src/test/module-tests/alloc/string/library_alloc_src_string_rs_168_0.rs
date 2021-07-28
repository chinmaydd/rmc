#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_168_0() {
        trait TraitExample {}

        impl<'a> TraitExample for &'a str {}

        fn example_func<A: TraitExample>(example_arg: A) {}

        let example_string = String::from("example_string");
        example_func(&example_string);
    }
    _doctest_main_library_alloc_src_string_rs_168_0()
}
