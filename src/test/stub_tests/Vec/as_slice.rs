include!{"../../rmc-prelude.rs"}

fn main() {
    fn as_slice_test() {
        use std::io::{self, Write};
        let buffer = rmc_vec![1, 2, 3, 5, 8];
        io::sink().write(buffer.as_slice()).unwrap();
    }

    as_slice_test();
}
