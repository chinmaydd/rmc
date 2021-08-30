// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// rmc --abs-type rmc main.rs --c-lib ../../stub/hashset.c

include!{"../../rmc-prelude.rs"}

fn main() {
    let mut h = HashSet::new();
    h.insert(10 as u32);
    assert!(h.contains(&10));
    assert!(!h.contains(&5));
    assert!(h.remove(10));
    assert!(!h.contains(&10));
}
