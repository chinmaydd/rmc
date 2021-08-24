// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod stub;

#[cfg(abs_type = "std")]
use std::vec::Vec;

#[cfg(abs_type = "c-ffi")]
use stub::c_vec::Vec;

#[cfg(abs_type = "no-back")]
use stub::noback_vec::Vec;

#[cfg(abs_type = "rmc")]
use stub::rmc_vec::Vec;

#[cfg(abs_type = "rmc")]
use stub::hashset::HashSet;

fn __VERIFIER_assume(cond: bool) {
    unimplemented!()
}

fn __VERIFIER_expect_fail(cond: bool, message: &str) {
    unimplemented!()
}

fn __nondet<T>() -> T {
    unimplemented!()
}
