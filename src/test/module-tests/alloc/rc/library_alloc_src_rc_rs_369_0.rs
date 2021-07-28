#![allow(unused_variables)]
#![deny(warnings)]
#![feature(arc_new_cyclic)]
#![allow(dead_code)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_369_0() {
        use std::rc::{Rc, Weak};

        struct Gadget {
            self_weak: Weak<Self>,
            // ... more fields
        }
        impl Gadget {
            pub fn new() -> Rc<Self> {
                Rc::new_cyclic(|self_weak| {
                    Gadget {
                        self_weak: self_weak.clone(), /* ... */
                    }
                })
            }
        }
    }
    _doctest_main_library_alloc_src_rc_rs_369_0()
}
