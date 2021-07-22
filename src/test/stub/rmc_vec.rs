// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This is the default vector abstraction which supports all methods and is generic
// over the type of the elements it can contain.
//
// The implementation is heavily inspired from the Rustonomicon and the Smack library.
extern crate libc;

use std::mem;
use std::marker::PhantomData;
use std::ptr::{copy, write, read};
use std::ops::{Deref, DerefMut};

pub struct RmcUnique<T: Sized> {
    ptr: *const T, // *const for variance
    _marker: PhantomData<T>,
}

impl<T: Sized> RmcUnique<T> {
    pub fn new(ptr: *mut T) -> Self {
        RmcUnique {
            ptr,
            _marker: Default::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }
}

// sized_realloc implements resizing of memory which stores the elements contained
// in the vector.
//
// TODO: This method is the fundamental bottleneck when resizing the array due to
// the expensive malloc + memcpy procedures. Its remains to be seen if this can be
// optimized using custom CBMC primitives.
fn sized_realloc(orig_ptr: *mut u8, orig_size: usize, new_size: usize) -> *mut u8 {
    unsafe {
        let result = libc::malloc(new_size) as *mut u8;
        libc::memcpy(
            result as *mut libc::c_void,
            orig_ptr as *mut libc::c_void,
            orig_size
        );
        result
    }
}

// RmcRawVec abstracts away common methods and functionality otherwise needed for
// RmcVec and RmcIter
struct RmcRawVec<T: Sized> {
    ptr: RmcUnique<T>,
    cap: usize,
}

impl<T: Sized> RmcRawVec<T> {
    fn new() -> Self {
        let elem_size = mem::size_of::<T>();
        // NOTE: (Mark. B) This default allocation size is important.
        // A default 0 size leads to complex solver queries for smaller vec operations
        let cap = 32;
        let ptr = unsafe { RmcUnique::new(libc::malloc(cap * elem_size) as *mut T) };
        RmcRawVec { ptr, cap }
    }

    fn new_with_capacity(cap: usize) -> Self {
        let elem_size = mem::size_of::<T>();
        let ptr = unsafe { RmcUnique::new(libc::malloc(cap * elem_size) as *mut T) };
        RmcRawVec { ptr, cap }
    }

    // grow() calls sized_realloc. By default, the new memory has twice the capacity
    // of the old one.
    fn grow(&mut self) {
        let elem_size = mem::size_of::<T>();
        let new_cap = 2 * self.cap;
        let ptr = sized_realloc(
            self.ptr.as_ptr() as *mut _,
            self.cap * elem_size,
            new_cap * elem_size
        );

        self.ptr = RmcUnique::new(ptr as *mut _);
        self.cap = new_cap;
    }

    fn needs_to_grow(&self, len: usize, additional: usize) -> bool {
        additional > self.cap - len
    }

    // NOTE: Implement fixes which handle large allocations
    fn reserve(&mut self, len: usize, additional: usize) {
        if self.needs_to_grow(len, additional) {
            // This function reserves space for atleast `additional` elements.
            let elem_size = mem::size_of::<T>();
            let ptr = sized_realloc(
                self.ptr.as_ptr() as *mut _,
                len * elem_size,
                (len + additional) * elem_size
            );

            self.ptr = RmcUnique::new(ptr as *mut _);
            self.cap = len + additional;
        }
    }

    fn capacity(&self) -> usize {
        self.cap
    }
}

// TODO: Implement this soundly.
impl<T: Sized> Drop for RmcRawVec<T> {
    fn drop(&mut self) {
        // unsafe { libc::free(self.ptr.ptr as *mut _) };
    }
}

// Vec abstraction
pub struct RmcVec<T: Sized> {
    buf: RmcRawVec<T>,
    len: usize,
}

impl<T: Sized> RmcVec<T> {
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new() -> Self {
        RmcVec {
            buf: RmcRawVec::new(),
            len: 0
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        RmcVec {
            buf: RmcRawVec::new_with_capacity(cap),
            len: 0
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            write(self.ptr().offset(self.len as isize), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(read(self.ptr().offset(self.len as isize))) }
        }
    }

    pub fn append(&mut self, other: &mut RmcVec<T>) {
        let mut i: usize = 0;
        let olen = other.len();
        let mut drain = RmcVec::new();
        while i < olen {
            drain.push(other.pop().unwrap());
            i += 1;
        }
        // Empty other
        i = 0;
        while i < olen {
            self.push(drain.pop().unwrap());
            i += 1;
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len);
        if self.cap() == self.len {
            self.buf.grow();
        }

        unsafe {
            if index < self.len {
                copy(
                    self.ptr().offset(index as isize),
                    self.ptr().offset(index as isize + 1),
                    self.len - index,
                );
            }
            write(self.ptr().offset(index as isize), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        unsafe {
            self.len -= 1;
            let result = read(self.ptr().offset(index as isize));
            copy(
                self.ptr().offset(index as isize + 1),
                self.ptr().offset(index as isize),
                self.len - index,
            );
            result
        }
    }

    pub fn extend<I: Iterator>(&mut self, iter: I) where I: Iterator<Item = T> {
        iter.for_each(move |element| {
            self.push(element);
        });
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(self.len, additional);
    }

    pub fn set_len(&mut self, new_len: usize) {
        assert!(new_len <= self.buf.capacity());

        self.len = new_len;
    }
}

impl<T: Sized + PartialEq> PartialEq for RmcVec<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for idx in 0..self.len {
            if self[idx] != other[idx] {
                return false
            }
        }

        return true;
    }
}

impl<T: Sized> IntoIterator for RmcVec<T> {
    type Item = T;
    type IntoIter = RmcIntoIter<T>;

    fn into_iter(self) -> RmcIntoIter<T> {
        unsafe {
            let iter = RmcRawValIter::new(&self);
            let buf = read(&self.buf);
            mem::forget(self);

            RmcIntoIter {
                iter,
                _buf: buf,
            }
        }
    }
}

impl<T: Default> Default for RmcVec<T> {
    fn default() -> Self {
        RmcVec::new()
    }
}

// TODO: Implement this soundly.
impl<T: Sized> Drop for RmcVec<T> {
    fn drop(&mut self) {
    }
}

// Coercion support into Deref allows us to benefit from operations on slice
// implemented in the standard library.
impl<T: Sized> Deref for RmcVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { ::std::slice::from_raw_parts(self.buf.ptr.ptr, self.len) }
    }
}

impl<T: Sized> DerefMut for RmcVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { ::std::slice::from_raw_parts_mut(self.buf.ptr.ptr as *mut T, self.len) }
    }
}

struct RmcRawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RmcRawValIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        RmcRawValIter {
            start: slice.as_ptr(),
            end: if mem::size_of::<T>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().offset(slice.len() as isize)
            },
        }
    }

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = read(self.start);
                self.start = if mem::size_of::<T>() == 0 {
                    (self.start as usize + 1) as *const _
                } else {
                    self.start.offset(1)
                };
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> Iterator for RmcRawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = read(self.start);
                self.start = if mem::size_of::<T>() == 0 {
                    (self.start as usize + 1) as *const _
                } else {
                    self.start.offset(1)
                };
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RmcRawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = if mem::size_of::<T>() == 0 {
                    (self.end as usize - 1) as *const _
                } else {
                    self.end.offset(-1)
                };
                Some(read(self.end))
            }
        }
    }
}

pub struct RmcIntoIter<T: Sized> {
    _buf: RmcRawVec<T>,
    iter: RmcRawValIter<T>,
}

impl<T: Sized> Iterator for RmcIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T: Sized> DoubleEndedIterator for RmcIntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

// TODO: Implement this soundly.
impl<T: Sized> Drop for RmcIntoIter<T> {
    fn drop(&mut self) {
    }
}

#[macro_export]
macro_rules! rmc_vec {
  ( $val:expr ; $count:expr ) =>
    ({
      let mut result = RmcVec::new();
      let mut i: usize = 0;
      while i < $count {
        result.push($val);
        i += 1;
      }
      result
    });
  ( $( $xs:expr ),* ) => {
    {
      let mut result = RmcVec::new();
      $(
        result.push($xs);
      )*
      result
    }
  };
}

macro_rules! __impl_slice_eq1 {
    ([$($vars:tt)*] $lhs:ty, $rhs:ty) => {
        impl<T, U, $($vars)*> PartialEq<$rhs> for $lhs
        where
            T: PartialEq<U>
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { self[..] == other[..] }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool { self[..] != other[..] }
        }
    }
}

__impl_slice_eq1! { [] RmcVec<T>, &[U] }
__impl_slice_eq1! { [] RmcVec<T>, &mut [U] }
__impl_slice_eq1! { [] &[T], RmcVec<U> }
__impl_slice_eq1! { [] &mut [T], RmcVec<U> }
__impl_slice_eq1! { [const N: usize] RmcVec<T>, [U; N] }
__impl_slice_eq1! { [const N: usize] RmcVec<T>, &[U; N] }
