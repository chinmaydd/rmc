// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
include!{"../../rmc-prelude.rs"}

fn main() {
    fn as_mut_slice_test() {
        use std::io::{self, Read};
        let mut buffer = rmc_rmc_vec![0; 3];
        io::repeat(0b101).read_exact(buffer.as_mut_slice()).unwrap();
    }

    as_mut_slice_test();
}
