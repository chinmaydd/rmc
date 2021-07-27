// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(rustc_private)]
include!{"../../rmc-prelude.rs"}

fn main() {
    let mut h = HashSet::new();
    h.insert(5);
    h.insert(6);
    assert!(h.contains(5));
    assert!(!h.contains(7));
}
