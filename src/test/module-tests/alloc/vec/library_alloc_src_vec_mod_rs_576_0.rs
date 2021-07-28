#![allow(unused_variables)]
#![deny(warnings)]
#![feature(allocator_api)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_576_0() {
        use std::alloc::System;

        let mut vec = Vec::with_capacity_in(10, System);

        // The vector contains no items, even though it has capacity for more
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        // These are all done without reallocating...
        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        // ...but this may make the vector reallocate
        vec.push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_576_0()
}
