use core::ops::{Index, IndexMut};

use crate::ringbuffer_trait::RingBuffer;

extern crate alloc;
// We need vecs so depend on alloc
use alloc::vec::Vec;

/// The RingBuffer struct.
///
/// # Example
/// ```
/// use ringbuffer::AllocRingBuffer;
/// use ringbuffer::RingBuffer;
///
/// let mut buffer = AllocRingBuffer::with_capacity(2);
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
#[derive(PartialEq, Eq, Debug)]
pub struct AllocRingBuffer<T> {
    buf: Vec<T>,
    cap: usize,
    index: usize,
}

/// The capacity of a RingBuffer created by new or default (`1024`).
pub const RINGBUFFER_DEFAULT_CAPACITY: usize = 1024;

impl<T: 'static + Default> RingBuffer<T> for AllocRingBuffer<T> {
    #[inline]
    fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    fn clear(&mut self) {
        self.buf.clear();
        self.index = 0;
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.cap
    }

    fn push(&mut self, e: T) {
        if self.buf.len() < self.capacity() {
            self.buf.push(e);
        } else {
            self.buf[self.index] = e;
        }

        self.index = (self.index + 1) % self.capacity()
    }

    impl_ringbuffer!(buf, index);
}

impl<T> AllocRingBuffer<T> {
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
}

// impl<T: Clone> IntoIterator for AllocRingBuffer<T> {
//     type Item = T;
//     type IntoIter = core::iter::Chain<core::slice::Iter<'_, T>, core::slice::Iter<'_, T>>;
//
//     fn into_iter(mut self) -> Self::IntoIter {
//         if self.index > self.cap / 2 {
//             // left owned, copy right
//             let r = self.buf[self.index..].iter().cloned();
//             self.buf.truncate(self.index);
//
//             self.buf.into_iter().chain(r)
//         } else {
//             // right owned, copy left
//             let l = self.buf[..self.index].iter().cloned();
//             let r = self.buf.into_iter().skip(self.index);
//             r.chain(l)
//         }
//     }
// }

impl<T> Default for AllocRingBuffer<T> {
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

impl<T> Index<usize> for AllocRingBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T> IndexMut<usize> for AllocRingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let b: AllocRingBuffer<u32> = AllocRingBuffer::default();
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.capacity());
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.buf.capacity());
        assert_eq!(b.cap, b.capacity());
        assert_eq!(b.buf.len(), b.len());
        assert_eq!(0, b.index);
        assert!(b.is_empty());
        assert!(b.buf.is_empty());
        assert_eq!(0, b.iter().count());
        assert_eq!(
            Vec::<u32>::with_capacity(RINGBUFFER_DEFAULT_CAPACITY),
            b.buf
        );
        assert_eq!(
            Vec::<u32>::with_capacity(RINGBUFFER_DEFAULT_CAPACITY),
            b.to_vec()
        );
    }

    #[test]
    fn test_default_capacity_constant() {
        // This is to prevent accidentally changing it.
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, 1024)
    }

    #[test]
    #[should_panic]
    fn test_index_bigger_than_length() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(2);

        b[2];
    }
}
