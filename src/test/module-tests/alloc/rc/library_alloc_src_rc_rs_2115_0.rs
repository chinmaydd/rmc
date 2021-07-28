#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_2115_0() {
        use std::rc::{Rc, Weak};

        let strong = Rc::new("hello".to_owned());
        let weak = Rc::downgrade(&strong);
        let raw = weak.into_raw();

        assert_eq!(1, Rc::weak_count(&strong));
        assert_eq!("hello", unsafe { &*raw });

        drop(unsafe { Weak::from_raw(raw) });
        assert_eq!(0, Rc::weak_count(&strong));
    }
    _doctest_main_library_alloc_src_rc_rs_2115_0()
}
