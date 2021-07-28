#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_974_0() {
        let mut s = String::with_capacity(10);
        s.push('a');
        s.push('b');

        // s now has a length of 2 and a capacity of 10
        assert_eq!(2, s.len());
        assert_eq!(10, s.capacity());

        // Since we already have an extra 8 capacity, calling this...
        s.reserve_exact(8);

        // ... doesn't actually increase.
        assert_eq!(10, s.capacity());
    }
    _doctest_main_library_alloc_src_string_rs_974_0()
}
