// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
include!{"../../rmc-prelude.rs"}

fn main() {
    fn as_ptr_test() {
        let x = rmc_vec![1, 2, 4];
        let x_ptr = x.as_ptr();

        unsafe {
            for i in 0..x.len() {
                assert_eq!(*x_ptr.add(i), 1 << i);
            }
        }
    }

    as_ptr_test()
}
