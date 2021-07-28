#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2329_0() {
        use std::rc::{Rc, Weak};

        struct Foo;

        impl Drop for Foo {
            fn drop(&mut self) {
                println!("dropped!");
            }
        }

        let foo = Rc::new(Foo);
        let weak_foo = Rc::downgrade(&foo);
        let other_weak_foo = Weak::clone(&weak_foo);

        drop(weak_foo); // Doesn't print anything
        drop(foo); // Prints "dropped!"

        assert!(other_weak_foo.upgrade().is_none());
    }
    _doctest_main_library_alloc_src_rc_rs_2329_0()
}
