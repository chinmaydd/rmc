#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_rc_rs_1184_0() {
        use std::any::Any;
        use std::rc::Rc;

        fn print_if_string(value: Rc<dyn Any>) {
            if let Ok(string) = value.downcast::<String>() {
                println!("String ({}): {}", string.len(), string);
            }
        }

        let my_string = "Hello World".to_string();
        print_if_string(Rc::new(my_string));
        print_if_string(Rc::new(0i8));
    }
    _doctest_main_library_alloc_src_rc_rs_1184_0()
}
