#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_188_0() {
        use std::sync::Arc;
        use std::thread;

        let five = Arc::new(5);

        for _ in 0..10 {
            let five = Arc::clone(&five);

            thread::spawn(move || {
                println!("{:?}", five);
            });
        }
    }
    _doctest_main_library_alloc_src_sync_rs_188_0()
}
