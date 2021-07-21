#![feature(rustc_private)]

extern crate libc;
use libc::{c_uint, size_t};
use std::marker::PhantomData;

extern "C" {
    fn ffi_new() -> *mut c_vec;
    fn ffi_push(ptr: *mut c_vec, elem: c_uint);
    fn ffi_cap(ptr: *mut c_vec) -> c_uint;
    fn ffi_len(ptr: *mut c_vec) -> c_uint;
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
    fn new() -> Self {
        unsafe {
            Vec {
                ptr: ffi_new(),
                phantom: Default::default()
            }
        }
    }

    // Ideally should be T?
    fn push(&mut self, elem: c_uint) {
        unsafe {
            ffi_push(self.ptr, elem);
        }
    }

    fn cap(&self) -> usize {
        unsafe {
            ffi_cap(self.ptr) as usize
        }
    }

    fn len(&self) -> usize {
        unsafe {
            ffi_len(self.ptr) as usize
        }
    }
}

fn main() {
    let mut v: Vec<u32> = Vec::new();
    v.push(10);
    assert!(v.len() == 1);
}
