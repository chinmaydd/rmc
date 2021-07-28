#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_1401_0() {
        use std::any::Any;

        fn print_if_string(value: Box<dyn Any + Send>) {
            if let Ok(string) = value.downcast::<String>() {
                println!("String ({}): {}", string.len(), string);
            }
        }

        let my_string = "Hello World".to_string();
        print_if_string(Box::new(my_string));
        print_if_string(Box::new(0i8));
    }
    _doctest_main_library_alloc_src_boxed_rs_1401_0()
}
