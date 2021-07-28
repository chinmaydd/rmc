#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_403_0() {
        let mut s = String::with_capacity(10);

        // The String contains no chars, even though it has capacity for more
        assert_eq!(s.len(), 0);

        // These are all done without reallocating...
        let cap = s.capacity();
        for _ in 0..10 {
            s.push('a');
        }

        assert_eq!(s.capacity(), cap);

        // ...but this may make the string reallocate
        s.push('a');
    }
    _doctest_main_library_alloc_src_string_rs_403_0()
}
