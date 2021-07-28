#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1127_0() {
        use std::rc::Rc;

        let mut data = Rc::new(75);
        let weak = Rc::downgrade(&data);

        assert!(75 == *data);
        assert!(75 == *weak.upgrade().unwrap());

        *Rc::make_mut(&mut data) += 1;

        assert!(76 == *data);
        assert!(weak.upgrade().is_none());
    }
    _doctest_main_library_alloc_src_rc_rs_1127_0()
}
