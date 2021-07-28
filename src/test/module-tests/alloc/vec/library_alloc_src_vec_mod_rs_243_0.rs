#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_243_0() {
        fn read_slice(slice: &[usize]) {
            // ...
        }

        let v = vec![0, 1];
        read_slice(&v);

        // ... and that's all!
        // you can also do it like this:
        let u: &[usize] = &v;
        // or like this:
        let u: &[_] = &v;
    }
    _doctest_main_library_alloc_src_vec_mod_rs_243_0()
}
