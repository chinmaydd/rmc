#![allow(unused_variables)]
#![deny(warnings)]
#![feature(arc_new_cyclic)]
#![allow(dead_code)]

fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_356_0() {
        use std::sync::{Arc, Weak};

        struct Foo {
            me: Weak<Foo>,
        }

        let foo = Arc::new_cyclic(|me| Foo { me: me.clone() });
    }
    _doctest_main_library_alloc_src_sync_rs_356_0()
}
