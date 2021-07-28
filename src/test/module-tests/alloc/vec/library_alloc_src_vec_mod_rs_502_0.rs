#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_502_0() {
        use std::mem;
        use std::ptr;

        let v = vec![1, 2, 3];

        // Prevent running `v`'s destructor so we are in complete control
        // of the allocation.
        let mut v = mem::ManuallyDrop::new(v);

        // Pull out the various important pieces of information about `v`
        let p = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();

        unsafe {
            // Overwrite memory with 4, 5, 6
            for i in 0..len as isize {
                ptr::write(p.offset(i), 4 + i);
            }

            // Put everything back together into a Vec
            let rebuilt = Vec::from_raw_parts(p, len, cap);
            assert_eq!(rebuilt, [4, 5, 6]);
        }
    }
    _doctest_main_library_alloc_src_vec_mod_rs_502_0()
}
