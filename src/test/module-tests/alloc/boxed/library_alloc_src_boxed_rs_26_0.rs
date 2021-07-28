#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_26_0() {
        #[derive(Debug)]
        enum List<T> {
            Cons(T, Box<List<T>>),
            Nil,
        }

        let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        println!("{:?}", list);
    }
    _doctest_main_library_alloc_src_boxed_rs_26_0()
}
