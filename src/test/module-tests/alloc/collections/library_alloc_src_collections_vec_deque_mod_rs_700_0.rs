#![allow(unused_variables)]
#![deny(warnings)]
#![feature(try_reserve)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_700_0() {
        use std::collections::TryReserveError;
        use std::collections::VecDeque;

        fn process_data(data: &[u32]) -> Result<VecDeque<u32>, TryReserveError> {
            let mut output = VecDeque::new();

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
    _doctest_main_library_alloc_src_collections_vec_deque_mod_rs_700_0()
}
