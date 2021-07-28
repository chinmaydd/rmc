#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1421_0() {
        use std::rc::Rc;

        struct Foo;

        impl Drop for Foo {
            fn drop(&mut self) {
                println!("dropped!");
            }
        }

        let foo = Rc::new(Foo);
        let foo2 = Rc::clone(&foo);

        drop(foo); // Doesn't print anything
        drop(foo2); // Prints "dropped!"
    }
    _doctest_main_library_alloc_src_rc_rs_1421_0()
}
