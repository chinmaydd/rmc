// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This is the default vector abstraction which supports all methods and is generic
// over the type of the elements it can contain.
//
// The implementation is heavily inspired from the Rustonomicon and the Smack library.
extern crate libc;

use std::mem;
use std::marker::PhantomData;
use std::ptr::{copy, write, read, replace, copy_nonoverlapping};
use std::ops::{Deref, DerefMut, FnMut, RangeBounds, Range};

pub struct RmcUnique<T> {
    ptr: *const T, // *const for variance
    _marker: PhantomData<T>,
}

impl<T> RmcUnique<T> {
    pub fn new(ptr: *mut T) -> Self {
        RmcUnique {
            ptr,
            _marker: Default::default(),
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }

    pub fn as_const_ptr(&self) -> *const T {
        self.ptr
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
// Vec and RmcIter
struct RmcRawVec<T> {
    ptr: RmcUnique<T>,
    cap: usize,
}

impl<T> RmcRawVec<T> {
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

    fn shrink_to_fit(&mut self, len: usize) {
        let elem_size = mem::size_of::<T>();
        let ptr = sized_realloc(
            self.ptr.as_ptr() as *mut _,
            self.cap * elem_size,
            len * elem_size
        );

        self.ptr = RmcUnique::new(ptr as *mut _);
        self.cap = len;
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
impl<T> Drop for RmcRawVec<T> {
    fn drop(&mut self) {
        // unsafe { libc::free(self.ptr.ptr as *mut _) };
    }
}

pub trait Allocator {
}

pub struct RmcAllocator {
}

impl RmcAllocator {
    pub fn new() -> Self {
        RmcAllocator {}
    }
}

impl Allocator for RmcAllocator {
}

impl Default for RmcAllocator {
    fn default() -> Self {
        RmcAllocator::new()
    }
}

// Vec abstraction
// Ideally A should implement Allocator and the default type assigned to it
// is Global. Currently stubbing it with type u64 since we are not really
// using it anywhere.
pub struct Vec<T, A : Allocator = RmcAllocator> {
    buf: RmcRawVec<T>,
    len: usize,
    allocator: A, // type A ideally implements allocator
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            buf: RmcRawVec::new(),
            len: 0,
            allocator: Default::default()
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::with_capacity_in(cap, RmcAllocator::new())
    }

    pub fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        let mut v = Vec {
            buf: RmcRawVec::new_with_capacity(capacity),
            len: 0,
            allocator: RmcAllocator::new()
        };
        unsafe {
            let mut curr_idx: isize = 0;
            while curr_idx < length as isize {
                v.push(read(ptr.offset(curr_idx)));
                curr_idx += 1;
            }
        }
        v
    }

}

impl <T, A: Allocator> Vec<T, A> {
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn allocator(&self) -> &A {
        assert!(false);
        unimplemented!()
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

    pub fn reserve_exact(&mut self, additional: usize) {
        self.buf.reserve(self.len, additional);
    }

    pub fn set_len(&mut self, new_len: usize) {
        assert!(new_len <= self.buf.capacity());

        self.len = new_len;
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr()
    }

    pub fn as_slice(&self) -> &[T] {
        self
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    pub fn as_ptr(&self) -> *const T {
        self.buf.ptr.as_const_ptr()
    }


    pub fn truncate(&mut self, len: usize) {
        // TODO: For soundness wrt standard library, call drop_in_place() for pointer
        // pointing to a slice of elements which were dropped.
        self.len = len;
    }

    // Clears the vector, removing all values.
    // This method has no effect on the allocated capacity of the vector
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool {
        assert!(false);
    }

    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K> {
        assert!(false);
    }

    // TODO:
    // pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    // where
    //     R: RangeBounds<usize> {
    //     assert!(false);
    // }

    // TODO:
    // pub fn drain_filter<F>(&mut self, filter: F) -> DrainFilter<'_, T, F, A>
    // where:
    //     F: FnMut(&mut T) -> bool {
    //     assert!(false);
    // }

    // TODO:
    // pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
    //     unimplemeted!()
    // }

    // TODO:
    // pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
    //     unimplemented!()
    // }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len;
        assert!(index < len);

        unsafe {
            let last = read(self.as_ptr().add(len - 1));
            let hole = self.as_mut_ptr().add(index);
            self.set_len(len - 1);
            replace(hole, last)
        }
    }
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    pub fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone {
        assert!(at < self.len);

        if at == 0 {
            return mem::replace(
                self,
                Vec::with_capacity_in(self.capacity(), self.allocator().clone())
            );
        }

        let other_len = self.len - at;
        let mut other = Vec::with_capacity_in(other_len, self.allocator().clone());

        unsafe {
            self.set_len(at);
            other.set_len(other_len);

            copy_nonoverlapping(self.as_ptr().add(at), other.as_mut_ptr(), other.len());
        }

        other
    }

    fn with_capacity_in(capacity: usize, allocator: A) -> Self {
        Vec {
            buf: RmcRawVec::new(),
            len: 0,
            allocator: allocator
        }
    }

    pub fn append(&mut self, other: &mut Vec<T, A>) {
        let mut i: usize = 0;
        let olen = other.len();
        let mut drain = Vec::new();
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

    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T {
        let len = self.len();

        if new_len > len {
            let additional = new_len - len;
            self.reserve(additional);

            let mut closure = f;

            unsafe {
                let mut ptr = self.as_mut_ptr().add(self.len());

                for _ in 0..additional {
                    write(ptr, closure());
                    self.len += 1;
                }
            }
        } else {
            self.truncate(new_len);
        }
    }

    pub fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        assert!(false);
        unimplemented!()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        if self.capacity() > min_capacity {
            let max = if self.len > min_capacity { self.len } else { min_capacity };
            self.buf.shrink_to_fit(max);
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool {
        assert!(false);
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn new_in(alloc: A) -> Self {
        Vec {
            buf: RmcRawVec::new(),
            len: 0,
            allocator: alloc
        }
    }

    // pub fn into_boxed_slice(mut self) -> Box<[T], A> {
    //     unsafe {
    //     }
    // }
}

impl<T: Clone, A: Allocator> Vec<T, A> {
    pub fn resize(&mut self, new_len: usize, value: T) {
        let len = self.len();

        if new_len > len {
            let additional = new_len - len;
            self.reserve(additional);

            unsafe {
                let mut ptr = self.as_mut_ptr().add(self.len());

                for _ in 0..additional {
                    write(ptr, value.clone());
                    self.len += 1;
                }
            }
        } else {
            self.truncate(new_len);
        }
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        for elem in other.iter() {
            self.push(elem.clone());
        }
    }

    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize> {
        assert!(false);
    }
}

impl<T: PartialEq, A: Allocator> Vec<T, A> {
    pub fn dedup(&mut self) {
        self.dedup_by(|a, b| a == b);
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for Vec<T, A> {
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

impl<T, A: Allocator> IntoIterator for Vec<T, A> {
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

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Vec::new()
    }
}

// TODO: Implement this soundly.
impl<T, A: Allocator> Drop for Vec<T, A> {
    fn drop(&mut self) {
    }
}

// Coercion support into Deref allows us to benefit from operations on slice
// implemented in the standard library.
impl<T, A: Allocator> Deref for Vec<T, A> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { ::std::slice::from_raw_parts(self.buf.ptr.ptr, self.len) }
    }
}

impl<T, A: Allocator> DerefMut for Vec<T, A> {
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
      let mut result = Vec::new();
      let mut i: usize = 0;
      while i < $count {
        result.push($val);
        i += 1;
      }
      result
    });
  ( $( $xs:expr ),* ) => {
    {
      let mut result = Vec::new();
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
            T: PartialEq<U>, A: Allocator
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { self[..] == other[..] }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool { self[..] != other[..] }
        }
    }
}

__impl_slice_eq1! { [A] Vec<T, A>, &[U] }
__impl_slice_eq1! { [A] Vec<T, A>, &mut [U] }
__impl_slice_eq1! { [A] &[T], Vec<U, A> }
__impl_slice_eq1! { [A] &mut [T], Vec<U, A> }
__impl_slice_eq1! { [A, const N: usize] Vec<T, A>, [U; N] }
__impl_slice_eq1! { [A, const N: usize] Vec<T, A>, &[U; N] }
