#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_boxed_rs_97_0() {
        #[repr(C)]
        pub struct Foo;

        #[no_mangle]
        pub extern "C" fn foo_new() -> Box<Foo> {
            Box::new(Foo)
        }

        #[no_mangle]
        pub extern "C" fn foo_delete(_: Option<Box<Foo>>) {}
    }
    _doctest_main_library_alloc_src_boxed_rs_97_0()
}
