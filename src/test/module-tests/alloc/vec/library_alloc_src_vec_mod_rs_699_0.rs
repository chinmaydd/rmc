#![allow(unused_variables)]
#![deny(warnings)]
#![feature(vec_into_raw_parts)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_699_0() {
        let v: Vec<i32> = vec![-1, 0, 1];

        let (ptr, len, cap) = v.into_raw_parts();

        let rebuilt = unsafe {
            // We can now make changes to the components, such as
            // transmuting the raw pointer to a compatible type.
            let ptr = ptr as *mut u32;

            Vec::from_raw_parts(ptr, len, cap)
        };
        assert_eq!(rebuilt, [4294967295, 0, 1]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_699_0()
}
