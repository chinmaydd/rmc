#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_978_0() {
        let mut vec = Vec::with_capacity(10);
        vec.extend([1, 2, 3]);

        assert_eq!(vec.capacity(), 10);
        let slice = vec.into_boxed_slice();
        assert_eq!(slice.into_vec().capacity(), 3);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_978_0()
}
