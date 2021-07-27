// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate libc;
use self::libc::c_uint;

// WIP: HashSet abstraction. Currently only implements the idea for a HashSet<u32> 
// as an experiment. This function uses uninterpreted functions to model a set().

extern "C" {
    fn ffi_insert(value: c_uint) -> c_uint;
    fn ffi_contains(value: c_uint) -> c_uint;
    fn ffi_remove(value: c_uint) -> c_uint;
}

pub struct HashSet {
    len: c_uint
}

// Currently only implemented for c_uint = u32
impl HashSet {
    pub fn new() -> Self {
        HashSet {
            len: 0
        }
    }

    pub fn with_capacity(_capacity: usize) -> Self {
        HashSet {
            len: 0
        }
    }

    // TODO: Need to figure out how to return bool
    pub fn insert(&mut self, value: c_uint) -> bool {
        unsafe {
            ffi_insert(value) != 0
        }
    }

    pub fn contains(&self, value: &c_uint) -> bool {
        unsafe {
            ffi_contains(*value) != 0
        }
    }

    // TODO: Need to figure out how to return bool
    // TODO: Need to figure out how to REMOVE
    pub fn remove(&mut self, value: c_uint) -> bool {
        unsafe {
            ffi_remove(value) != 0
        }
    }
}
