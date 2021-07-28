#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2070_0() {
        use std::ptr;
        use std::rc::Rc;

        let strong = Rc::new("hello".to_owned());
        let weak = Rc::downgrade(&strong);
        // Both point to the same object
        assert!(ptr::eq(&*strong, weak.as_ptr()));
        // The strong here keeps it alive, so we can still access the object.
        assert_eq!("hello", unsafe { &*weak.as_ptr() });

        drop(strong);
        // But not any more. We can do weak.as_ptr(), but accessing the pointer would lead to
        // undefined behaviour.
        // assert_eq!("hello", unsafe { &*weak.as_ptr() });
    }
    _doctest_main_library_alloc_src_rc_rs_2070_0()
}
