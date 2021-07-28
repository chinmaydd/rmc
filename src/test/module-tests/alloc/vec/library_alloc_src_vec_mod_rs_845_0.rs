#![allow(unused_variables)]
#![deny(warnings)]
#![feature(try_reserve)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_845_0() {
        use std::collections::TryReserveError;

        fn process_data(data: &[u32]) -> Result<Vec<u32>, TryReserveError> {
            let mut output = Vec::new();

            // Pre-reserve the memory, exiting if we can't
            output.try_reserve(data.len())?;

            // Now we know this can't OOM in the middle of our complex work
            output.extend(data.iter().map(|&val| {
                val * 2 + 5 // very complicated
            }));

            Ok(output)
        }
        process_data(&[1, 2, 3]).expect("why is the test harness OOMing on 12 bytes?");
    }
    _doctest_main_library_alloc_src_vec_mod_rs_845_0()
}
