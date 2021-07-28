#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1717_0() {
        use std::sync::{Arc, Weak};

        let strong = Arc::new("hello".to_owned());
        let weak = Arc::downgrade(&strong);
        let raw = weak.into_raw();

        assert_eq!(1, Arc::weak_count(&strong));
        assert_eq!("hello", unsafe { &*raw });

        drop(unsafe { Weak::from_raw(raw) });
        assert_eq!(0, Arc::weak_count(&strong));
    }
    _doctest_main_library_alloc_src_sync_rs_1717_0()
}
