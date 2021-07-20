#![feature(rustc_private)]
extern crate libc;
use libc::c_uint;

extern "C" {
    fn ffi_insert(value: c_uint) -> c_uint;
    fn ffi_contains(value: c_uint) -> c_uint;
    fn ffi_remove(value: c_uint) -> c_uint;
}

pub struct HashSet {
    len: c_uint
}

// Currently only implemented for c_uint
impl HashSet {
    fn new() -> Self {
        HashSet {
            len: 0
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        HashSet {
            len: 0
        }
    }

    // TODO: Need to figure out how to return bool
    fn insert(&mut self, value: c_uint) -> bool {
        unsafe {
            ffi_insert(value) != 0
        }
    }

    fn contains(&self, value: c_uint) -> bool {
        unsafe {
            ffi_contains(value) != 0
        }
    }

    // TODO: Need to figure out how to return bool
    // TODO: Need to figure out how to REMOVE
    fn remove(&mut self, value: c_uint) -> bool {
        unsafe {
            ffi_remove(value) != 0
        }
    }
}

fn main() {
    let mut h: HashSet = HashSet::new();
    h.insert(5);
    h.insert(6);
    assert!(h.contains(5));
    assert!(h.contains(6));
    assert!(!h.contains(7));
}
