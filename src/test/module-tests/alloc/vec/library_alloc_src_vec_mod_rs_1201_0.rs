#![allow(unused_variables)]
#![deny(warnings)]
#![allow(dead_code)]
// This is just a minimal skeleton for the doc example;
// don't use this as a starting point for a real library.
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_1201_0() {
        pub struct StreamWrapper {
            strm: *mut std::ffi::c_void,
        }
        const Z_OK: i32 = 0;
        extern "C" {
            fn deflateGetDictionary(
                strm: *mut std::ffi::c_void,
                dictionary: *mut u8,
                dictLength: *mut usize,
            ) -> i32;
        }
        impl StreamWrapper {
            pub fn get_dictionary(&self) -> Option<Vec<u8>> {
                // Per the FFI method's docs, "32768 bytes is always enough".
                let mut dict = Vec::with_capacity(32_768);
                let mut dict_length = 0;
                // SAFETY: When `deflateGetDictionary` returns `Z_OK`, it holds that:
                // 1. `dict_length` elements were initialized.
                // 2. `dict_length` <= the capacity (32_768)
                // which makes `set_len` safe to call.
                unsafe {
                    // Make the FFI call...
                    let r = deflateGetDictionary(self.strm, dict.as_mut_ptr(), &mut dict_length);
                    if r == Z_OK {
                        // ...and update the length to what was initialized.
                        dict.set_len(dict_length);
                        Some(dict)
                    } else {
                        None
                    }
                }
            }
        }
    }
    _doctest_main_library_alloc_src_vec_mod_rs_1201_0()
}
