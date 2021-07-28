#![allow(unused_variables)]
#![deny(warnings)]
#![feature(try_reserve)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_1009_0() {
        use std::collections::TryReserveError;

        fn process_data(data: &str) -> Result<String, TryReserveError> {
            let mut output = String::new();

            // Pre-reserve the memory, exiting if we can't
            output.try_reserve(data.len())?;

            // Now we know this can't OOM in the middle of our complex work
            output.push_str(data);

            Ok(output)
        }
        process_data("rust").expect("why is the test harness OOMing on 4 bytes?");
    }
    _doctest_main_library_alloc_src_string_rs_1009_0()
}
