// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(rustc_private)]
include!{"../rmc-prelude.rs"}

fn main() {
    let mut v: RmcVec<u32> = RmcVec::new();
    v.push(__nondet());
    v.push(__nondet());
    assert!(v.len() == 2);
}
