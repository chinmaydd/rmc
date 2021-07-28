#![allow(unused_variables)]
#![deny(warnings)]
#![feature(raw_vec_internals)]
extern crate alloc;
use alloc::raw_vec::RawVec;
use std::ptr;
struct MyVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T: Clone> MyVec<T> {
    pub fn push_all(&mut self, elems: &[T]) {
        self.buf.reserve(self.len, elems.len());
        // reserve would have aborted or panicked if the len exceeded
        // `isize::MAX` so this is safe to do unchecked now.
        for x in elems {
            unsafe {
                ptr::write(self.buf.ptr().add(self.len), x.clone());
            }
            self.len += 1;
        }
    }
}
fn main() {
    let mut vector = MyVec {
        buf: RawVec::new(),
        len: 0,
    };
    vector.push_all(&[1, 3, 5, 7, 9]);
}
