#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_slice_rs_713_0() {
        #[allow(dead_code)]
        pub struct Foo(Vec<u32>, Vec<String>);

        impl std::borrow::Borrow<[u32]> for Foo {
            fn borrow(&self) -> &[u32] {
                &self.0
            }
        }

        impl std::borrow::Borrow<[String]> for Foo {
            fn borrow(&self) -> &[String] {
                &self.1
            }
        }
    }
    _doctest_main_library_alloc_src_slice_rs_713_0()
}
