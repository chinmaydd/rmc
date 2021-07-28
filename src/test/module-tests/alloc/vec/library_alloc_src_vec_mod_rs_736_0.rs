#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api, vec_into_raw_parts)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_736_0() {
        use std::alloc::System;

        let mut v: Vec<i32, System> = Vec::new_in(System);
        v.push(-1);
        v.push(0);
        v.push(1);

        let (ptr, len, cap, alloc) = v.into_raw_parts_with_alloc();

        let rebuilt = unsafe {
            // We can now make changes to the components, such as
            // transmuting the raw pointer to a compatible type.
            let ptr = ptr as *mut u32;

            Vec::from_raw_parts_in(ptr, len, cap, alloc)
        };
        assert_eq!(rebuilt, [4294967295, 0, 1]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_736_0()
}
