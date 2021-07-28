#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2158_0() {
        use std::rc::{Rc, Weak};

        let strong = Rc::new("hello".to_owned());

        let raw_1 = Rc::downgrade(&strong).into_raw();
        let raw_2 = Rc::downgrade(&strong).into_raw();

        assert_eq!(2, Rc::weak_count(&strong));

        assert_eq!(
            "hello",
            &*unsafe { Weak::from_raw(raw_1) }.upgrade().unwrap()
        );
        assert_eq!(1, Rc::weak_count(&strong));

        drop(strong);

        // Decrement the last weak count.
        assert!(unsafe { Weak::from_raw(raw_2) }.upgrade().is_none());
    }
    _doctest_main_library_alloc_src_rc_rs_2158_0()
}
