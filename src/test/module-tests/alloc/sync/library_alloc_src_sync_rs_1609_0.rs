#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_sync_rs_1609_0() {
        use std::any::Any;
        use std::sync::Arc;

        fn print_if_string(value: Arc<dyn Any + Send + Sync>) {
            if let Ok(string) = value.downcast::<String>() {
                println!("String ({}): {}", string.len(), string);
            }
        }

        let my_string = "Hello World".to_string();
        print_if_string(Arc::new(my_string));
        print_if_string(Arc::new(0i8));
    }
    _doctest_main_library_alloc_src_sync_rs_1609_0()
}
