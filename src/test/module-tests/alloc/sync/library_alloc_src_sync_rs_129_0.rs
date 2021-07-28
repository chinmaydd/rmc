#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_129_0() {
        use std::sync::Arc;
        let foo = Arc::new(vec![1.0, 2.0, 3.0]);
        // The two syntaxes below are equivalent.
        let a = foo.clone();
        let b = Arc::clone(&foo);
        // a, b, and foo are all Arcs that point to the same memory location
    }
    _doctest_main_library_alloc_src_sync_rs_129_0()
}
