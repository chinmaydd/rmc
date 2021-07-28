#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_207_0() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        use std::thread;

        let val = Arc::new(AtomicUsize::new(5));

        for _ in 0..10 {
            let val = Arc::clone(&val);

            thread::spawn(move || {
                let v = val.fetch_add(1, Ordering::SeqCst);
                println!("{:?}", v);
            });
        }
    }
    _doctest_main_library_alloc_src_sync_rs_207_0()
}
