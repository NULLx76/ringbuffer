#![no_std]

use core::iter::Chain;
use core::slice::Iter as SliceIter;
use core::slice::IterMut as SliceIterMut;
use core::ops::{Index, IndexMut};

// We need vecs so depend on alloc
extern crate alloc;
use alloc::vec::Vec;

/// The RingBuffer struct.
///
/// # Example
/// ```
/// use ringbuffer::RingBuffer;
///
/// let mut buffer = RingBuffer::with_capacity(2);
///
/// // First entry of the buffer is now 5.
/// buffer.push(5);
///
/// assert_eq!(buffer[0], 5);
///
/// // Second entry is now 42.
/// buffer.push(42);
///
/// // Because capacity is reached the next push will be the first item of the buffer.
/// buffer.push(1);
/// assert_eq!(buffer[0], 1);
/// ```
#[derive(PartialEq,Debug)]
pub struct RingBuffer<T> {
    #[cfg(not(test))]
    buf: Vec<T>,
    #[cfg(not(test))]
    cap: usize,
    #[cfg(not(test))]
    index: usize,

    // Make the fields public for testing purposes
    #[cfg(test)]
    pub buf: Vec<T>,
    #[cfg(test)]
    pub cap: usize,
    #[cfg(test)]
    pub index: usize,
}

/// The type returned by
/// [iter](struct.RingBuffer.html#method.iter).
pub type Iter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;
/// The type returned by
/// [iter_mut](struct.RingBuffer.html#method.iter_mut).
pub type IterMut<'a, T> = Chain<SliceIterMut<'a, T>, SliceIterMut<'a, T>>;

/// The capacity of a RingBuffer created by new or default (`1024`).
pub const RINGBUFFER_DEFAULT_CAPACITY: usize = 1024;

impl<T> RingBuffer<T> {

    /// Creates a RingBuffer with a certain capacity.
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "Capacity must be greater than zero");

        Self {
            buf: Vec::with_capacity(cap),
            cap,
            index: 0,
        }
    }

    /// Creates a RingBuffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the length of the internal buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns true if the buffer is empty, some value between 0 and capacity.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Empties the buffer.
    #[inline]
    pub fn clear(&mut self) {
        self.buf.clear();
        self.index = 0;
    }

    /// Returns the capacity of the buffer.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Pushes a value onto the buffer. Cycles around if capacity is reached.
    pub fn push(&mut self, e: T) {
        if self.buf.len() < self.capacity() {
            self.buf.push(e);
        } else {
            self.buf[self.index] = e;
        }

        self.index = (self.index + 1) % self.capacity()
    }

    /// Returns the value at the current index.
    /// This is the value that will be overwritten by the next push.
    pub fn peek(&self) -> Option<&T> {
        self.buf.get(self.index)
    }

    /// Creates an iterator over the buffer starting from the latest push.
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        let (l, r) = self.buf.split_at(self.index);
        r.iter().chain(l.iter())
    }

    ///  Creates a mutable iterator over the buffer starting from the latest push.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (l, r) = self.buf.split_at_mut(self.index);
        r.iter_mut().chain(l.iter_mut())
    }

    /// Converts the buffer to an vector.
    #[inline]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Copy,
    {
        self.iter().copied().collect()
    }
}

impl<T> Default for RingBuffer<T> {

    /// Creates a buffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    fn default() -> Self {
        let cap = RINGBUFFER_DEFAULT_CAPACITY;
        Self {
            buf: Vec::with_capacity(cap),
            cap,
            index: 0,
        }
    }
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Enable std in tests
    extern crate std;
    use std::vec;

    #[test]
    fn test_default() {
        let b: RingBuffer<u32> = RingBuffer::default();
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.capacity());
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.buf.capacity());
        assert_eq!(b.cap, b.capacity());
        assert_eq!(b.buf.len(), b.len());
        assert_eq!(0, b.index);
        assert!(b.is_empty());
        assert!(b.buf.is_empty());
        assert_eq!(0, b.iter().count());
        assert_eq!(Vec::<u32>::with_capacity(RINGBUFFER_DEFAULT_CAPACITY), b.buf);
        assert_eq!(Vec::<u32>::with_capacity(RINGBUFFER_DEFAULT_CAPACITY), b.to_vec());
    }

    #[test]
    fn test_default_eq_new() {
        assert_eq!(RingBuffer::<u32>::default(), RingBuffer::<u32>::new())
    }

    #[test]
    #[should_panic]
    fn test_no_empty() {
        RingBuffer::<u32>::with_capacity(0);
    }

    #[test]
    fn test_len() {
        let mut b = RingBuffer::<u32>::new();
        assert_eq!(0, b.len());
        b.push(1);
        assert_eq!(1, b.len());
        b.push(2);
        assert_eq!(2, b.len())
    }

    #[test]
    fn test_len_wrap() {
        let mut b = RingBuffer::<u32>::with_capacity(2);
        assert_eq!(0, b.len());
        b.push(1);
        assert_eq!(1, b.len());
        b.push(2);
        assert_eq!(2, b.len());
        // Now we are wrapping
        b.push(3);
        assert_eq!(2, b.len());
        b.push(4);
        assert_eq!(2, b.len());
    }

    #[test]
    fn test_clear() {
        let mut b = RingBuffer::<u32>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        b.clear();
        assert!(b.is_empty());
        assert_eq!(0, b.len());
        assert_eq!(0, b.buf.len());
    }

    #[test]
    fn test_empty() {
        let mut b = RingBuffer::<u32>::new();
        assert!(b.is_empty());
        b.push(1);
        b.push(2);
        b.push(3);
        assert_ne!(b.is_empty(), true);

        b.clear();
        assert!(b.is_empty());
        assert_eq!(0, b.len());
        assert_eq!(0, b.buf.len());
    }

    #[test]
    fn test_iter() {
        let mut b = RingBuffer::<u32>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        let mut iter = b.iter();
        assert_eq!(&1u32, iter.next().unwrap());
        assert_eq!(&2u32, iter.next().unwrap());
        assert_eq!(&3u32, iter.next().unwrap());
    }

    #[test]
    fn test_iter_wrap() {
        let mut b = RingBuffer::<u32>::with_capacity(2);
        b.push(1);
        b.push(2);
        // Wrap
        b.push(3);

        let mut iter = b.iter();
        assert_eq!(&2u32, iter.next().unwrap());
        assert_eq!(&3u32, iter.next().unwrap());
    }

    #[test]
    fn test_iter_mut() {
        let mut b = RingBuffer::<u32>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        for el in  b.iter_mut() {
            *el += 1;
        }

        assert_eq!(vec![2,3,4], b.to_vec())
    }

    #[test]
    fn test_iter_mut_wrap() {
        let mut b = RingBuffer::<u32>::with_capacity(2);
        b.push(1);
        b.push(2);
        b.push(3);

        for el in b.iter_mut() {
            *el += 1;
        }

        assert_eq!(vec![3,4], b.to_vec())
    }

    #[test]
    fn test_to_vec() {
        let mut b = RingBuffer::<u32>::with_capacity(3);
        b.push(1);
        b.push(2);
        b.push(3);

        assert_eq!(vec![1,2,3], b.to_vec())
    }

    #[test]
    fn test_to_vec_wrap() {
        let mut b = RingBuffer::<u32>::with_capacity(2);
        b.push(1);
        b.push(2);
        // Wrap
        b.push(3);

        assert_eq!(vec![2,3], b.to_vec())
    }

    #[test]
    fn test_index() {
        let mut b = RingBuffer::with_capacity(2);
        b.push(2);

        assert_eq!(b[0], 2)
    }

    #[test]
    fn test_index_mut() {
        let mut b = RingBuffer::with_capacity(2);
        b.push(2);

        assert_eq!(b[0], 2);

        b[0] = 5;

        assert_eq!(b[0], 5);
    }

    #[test]
    #[should_panic]
    fn test_index_bigger_than_length() {
        let mut b = RingBuffer::with_capacity(2);
        b.push(2);

        b[2];
    }

    #[test]
    fn test_peek_some() {
        let mut b = RingBuffer::with_capacity(2);
        b.push(1);
        b.push(2);

        assert_eq!(b.peek(),Some(&1));
    }

    #[test]
    fn test_peek_none() {
        let mut b = RingBuffer::with_capacity(2);
        b.push(1);

        assert_eq!(b.peek(),None);
    }
}
