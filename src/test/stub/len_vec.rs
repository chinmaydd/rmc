// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate libc;

use std::mem;
use std::ptr::{write, read};

// Abstraction which tracks only the length and the last element of the vector.
pub struct Vec<T> {
    len: usize,
    last: Option<*mut T>,
}

impl<T> Vec<T> {
    fn drain(&mut self) {
        self.len = 0;
        self.last = None;
    }

    fn update_last(&mut self, elem: T) {
        if self.len == 0 {
            let elem_size = mem::size_of::<T>();
            let ptr = unsafe { libc::malloc(elem_size) as *mut T };
            self.last = Some(ptr);
        }

        unsafe {
            write(self.last.unwrap().offset(0), elem);
        }
    }

    pub fn new() -> Vec<T> {
        Vec {
            len: 0,
            last: None
        }
    }

    pub fn with_capacity(_: usize) -> Self {
        Vec {
            len: 0,
            last: None
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == 0 {
            let elem_size = mem::size_of::<T>();
            let ptr = unsafe { libc::malloc(elem_size) as *mut T };
            self.last = Some(ptr);
        }
        self.len += 1;
        unsafe {
            write(self.last.unwrap().offset(0), elem);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(read(self.last.unwrap().offset(0)))
            }
        }
    }

    pub fn append(&mut self, other: &mut Vec<T>) {
        if other.len() != 0 {
            self.update_last(other.pop().unwrap());
            self.len += other.len();
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len);

        if index == self.len {
            self.push(elem);
        } else {
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        if index == self.len - 1 {
            panic!("This operation is unsound for the current abstraction!");
        } else {
            self.pop().unwrap()
        }
    }

    pub fn extend<I: Iterator>(&mut self, iter: I) where I: Iterator<Item = T> {
        let mut last = None;
        let mut other_len = 0;
        for value in iter {
            self.len += 1;
            last = Some(value);
            other_len += 1;
        }

        if other_len != 0 {
            self.update_last(last.unwrap());
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn reserve(&mut self, _: usize) {
    }

    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

// TODO: Implement this soundly.
impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
    }
}
