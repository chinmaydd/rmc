// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate libc;

use self::libc::{c_uint, size_t};
use std::marker::PhantomData;

// Abstraction which implements Vec operations using CBMC C primitives by only
// implementing skeleton functions in Rust and exporting all core functionality
// to C.

extern "C" {
    fn ffi_new() -> *mut c_vec;
    fn ffi_push(ptr: *mut c_vec, elem: c_uint);
    fn ffi_cap(ptr: *mut c_vec) -> c_uint;
    fn ffi_len(ptr: *mut c_vec) -> c_uint;
    fn ffi_with_capacity(cap: size_t) -> *mut c_vec;
    fn ffi_pop(ptr: *mut c_vec) -> c_uint;
    fn ffi_append(ptr1: *mut c_vec, ptr2: *mut c_vec);
    fn ffi_insert(ptr: *mut c_vec, index: size_t, elem: c_uint);
}

#[repr(C)]
pub struct c_vec {
    mem: *mut c_uint,
    len: size_t,
    capacity: size_t
}

pub struct Vec<T> {
    ptr: *mut c_vec,
    phantom: PhantomData<T>
}

impl<T> Vec<T> {
    pub fn ptr(&mut self) -> *mut c_vec {
        return self.ptr;
    }

    pub fn new() -> Self {
        unsafe {
            Vec {
                ptr: ffi_new(),
                phantom: Default::default()
            }
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        unsafe {
            Vec {
                ptr: ffi_with_capacity(cap),
                phantom: Default::default()
            }
        }
    }

    pub fn push(&mut self, elem: c_uint) {
        unsafe {
            ffi_push(self.ptr, elem);
        }
    }

    pub fn pop(&mut self) -> Option<c_uint> {
        if self.len() == 0 {
            None
        } else {
            unsafe {
                Some(ffi_pop(self.ptr))
            }
        }
    }

    pub fn append(&mut self, other: &mut Vec<T>) {
        unsafe {
            ffi_append(self.ptr, other.ptr());
        }
    }

    pub fn insert(&mut self, index: usize, elem: c_uint) {
        unsafe {
            ffi_insert(self.ptr, index, elem);
        }
    }

    pub fn cap(&self) -> usize {
        unsafe {
            ffi_cap(self.ptr) as usize
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            ffi_len(self.ptr) as usize
        }
    }
}
