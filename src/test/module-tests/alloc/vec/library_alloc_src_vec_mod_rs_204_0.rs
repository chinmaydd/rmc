#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_204_0() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            // Prints 3, 2, 1
            println!("{}", top);
        }
    }
    _doctest_main_library_alloc_src_vec_mod_rs_204_0()
}
