#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1540_0() {
        use std::sync::Arc;

        struct Foo;

        impl Drop for Foo {
            fn drop(&mut self) {
                println!("dropped!");
            }
        }

        let foo = Arc::new(Foo);
        let foo2 = Arc::clone(&foo);

        drop(foo); // Doesn't print anything
        drop(foo2); // Prints "dropped!"
    }
    _doctest_main_library_alloc_src_sync_rs_1540_0()
}
