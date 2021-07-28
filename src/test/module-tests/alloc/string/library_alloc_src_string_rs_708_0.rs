#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_708_0() {
        use std::mem;

        unsafe {
            let s = String::from("hello");

            // Prevent automatically dropping the String's data
            let mut s = mem::ManuallyDrop::new(s);

            let ptr = s.as_mut_ptr();
            let len = s.len();
            let capacity = s.capacity();

            let s = String::from_raw_parts(ptr, len, capacity);

            assert_eq!(String::from("hello"), s);
        }
    }
    _doctest_main_library_alloc_src_string_rs_708_0()
}
