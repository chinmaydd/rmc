#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1109_0() {
        use std::rc::Rc;

        let mut data = Rc::new(5);

        *Rc::make_mut(&mut data) += 1; // Won't clone anything
        let mut other_data = Rc::clone(&data); // Won't clone inner data
        *Rc::make_mut(&mut data) += 1; // Clones inner data
        *Rc::make_mut(&mut data) += 1; // Won't clone anything
        *Rc::make_mut(&mut other_data) *= 2; // Won't clone anything

        // Now `data` and `other_data` point to different allocations.
        assert_eq!(*data, 8);
        assert_eq!(*other_data, 12);
    }
    _doctest_main_library_alloc_src_rc_rs_1109_0()
}
