// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate libc;
use self::libc::{c_int, c_uint};

// WIP: HashSet abstraction. Currently only implements the idea for a HashSet<u32> 
// as an experiment. This function uses uninterpreted functions to model a set().

extern "C" {
    fn ffi_new() -> *mut c_hashset;
    fn ffi_insert(ptr: *mut c_hashset, value: c_uint) -> c_uint;
    fn ffi_contains(ptr: *mut c_hashset, value: c_uint) -> c_uint;
    fn ffi_remove(ptr: *mut c_hashset, value: c_uint) -> c_uint;
}

#[repr(C)]
pub struct c_hashset {
    domain: [c_int; u32::MAX as usize],
    counter: c_uint
}

pub struct HashSet {
    ptr: *mut c_hashset,
}

// Currently only implemented for c_uint = u32
impl HashSet {
    pub fn new() -> Self {
        unsafe {
            HashSet {
                ptr: ffi_new(),
            }
        }
    }

    // TODO: Need to figure out how to return bool
    pub fn insert(&mut self, value: c_uint) -> bool {
        unsafe {
            ffi_insert(self.ptr, value) != 0
        }
    }

    pub fn contains(&self, value: &c_uint) -> bool {
        unsafe {
            ffi_contains(self.ptr, *value) != 0
        }
    }

    // TODO: Need to figure out how to return bool
    // TODO: Need to figure out how to REMOVE
    pub fn remove(&mut self, value: c_uint) -> bool {
        unsafe {
            ffi_remove(self.ptr, value) != 0
        }
    }
}
