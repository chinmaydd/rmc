// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate libc;

use std::marker::PhantomData;

// Abstraction which tracks only the length of the vector and does not contain
// a backing store.
pub struct Vec<T> {
    len: usize,
    capacity: usize,
    _marker: PhantomData<T>,
}

impl<T> Vec<T> {
    fn drain(&mut self) {
        self.len = 0;
    }

    fn update_last(&mut self, elem: T) {
        self.len = 1;
    }

    pub fn new() -> Vec<T> {
        Vec {
            len: 0,
            capacity: 0,
            _marker: Default::default()
        }
    }

    pub fn with_capacity(_: usize) -> Self {
        Vec {
            len: 0,
            capacity: 0,
            _marker: Default::default()
        }
    }

    pub fn push(&mut self, elem: T) {
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unimplemented!()
        }
    }

    pub fn append(&mut self, other: &mut Vec<T>) {
        let new_len = self.len + other.len;
        if self.capacity < new_len {
            self.capacity = new_len;
        }

        assert!(self.capacity >= new_len);
        self.len = new_len;
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len);

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        unimplemented!()
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
