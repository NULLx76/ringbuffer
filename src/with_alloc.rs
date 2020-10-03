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

impl<'a, 'b,  T: 'static + Default> RingBuffer<'a, 'b, T> for AllocRingBuffer<T> {
    type Iter = core::iter::Chain<core::slice::Iter<'a, T>, core::slice::Iter<'a, T>>;
    type IterMut = core::iter::Chain<core::slice::IterMut<'b, T>, core::slice::IterMut<'b, T>>;

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

    #[inline]
    fn peek(&self) -> Option<&T> {
        self.buf.get(self.index)
    }

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        let (l, r) = self.buf.split_at(self.index);
        r.iter().chain(l.iter())
    }

    #[inline]
    fn iter_mut(&'b mut self) -> Self::IterMut {
        let (l, r) = self.buf.split_at_mut(self.index);
        r.iter_mut().chain(l.iter_mut())
    }

    #[inline]
    fn as_vec(&self) -> Vec<&T> {
        self.iter().collect()
    }
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

    // Use alloc in tests
    use alloc::vec;

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
    fn test_default_eq_new() {
        assert_eq!(AllocRingBuffer::<u32>::default(), AllocRingBuffer::<u32>::new())
    }

    #[test]
    #[should_panic]
    fn test_no_empty() {
        AllocRingBuffer::<u32>::with_capacity(0);
    }

    #[test]
    fn test_len() {
        let mut b = AllocRingBuffer::new();
        assert_eq!(0, b.len());
        b.push(1);
        assert_eq!(1, b.len());
        b.push(2);
        assert_eq!(2, b.len())
    }

    #[test]
    fn test_len_wrap() {
        let mut b = AllocRingBuffer::with_capacity(2);
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
        let mut b = AllocRingBuffer::new();
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
        let mut b = AllocRingBuffer::new();
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
        let mut b = AllocRingBuffer::new();
        b.push(1);
        b.push(2);
        b.push(3);

        let mut iter = b.iter();
        assert_eq!(&1, iter.next().unwrap());
        assert_eq!(&2, iter.next().unwrap());
        assert_eq!(&3, iter.next().unwrap());
    }

    #[test]
    fn test_double_iter() {
        let mut b = AllocRingBuffer::new();
        b.push(1);
        b.push(2);
        b.push(3);

        let mut iter1 = b.iter();
        let mut iter2 = b.iter();

        assert_eq!(&1, iter1.next().unwrap());
        assert_eq!(&2, iter1.next().unwrap());
        assert_eq!(&3, iter1.next().unwrap());
        assert_eq!(&1, iter2.next().unwrap());
        assert_eq!(&2, iter2.next().unwrap());
        assert_eq!(&3, iter2.next().unwrap());
    }

    #[test]
    fn test_iter_wrap() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(1);
        b.push(2);
        // Wrap
        b.push(3);

        let mut iter = b.iter();
        assert_eq!(&2, iter.next().unwrap());
        assert_eq!(&3, iter.next().unwrap());
    }

    #[test]
    fn test_iter_mut() {
        let mut b = AllocRingBuffer::new();
        b.push(1);
        b.push(2);
        b.push(3);

        for el in b.iter_mut() {
            *el += 1;
        }

        assert_eq!(vec![2, 3, 4], b.to_vec())
    }

    #[test]
    fn test_iter_mut_wrap() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(1);
        b.push(2);
        b.push(3);

        for el in b.iter_mut() {
            *el += 1;
        }

        assert_eq!(vec![3, 4], b.to_vec())
    }

    #[test]
    fn test_to_vec() {
        let mut b = AllocRingBuffer::with_capacity(3);
        b.push(1);
        b.push(2);
        b.push(3);

        assert_eq!(vec![1, 2, 3], b.to_vec())
    }

    #[test]
    fn test_to_vec_wrap() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(1);
        b.push(2);
        // Wrap
        b.push(3);

        assert_eq!(vec![2, 3], b.to_vec())
    }

    #[test]
    fn test_index() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(2);

        assert_eq!(b[0], 2)
    }

    #[test]
    fn test_index_mut() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(2);

        assert_eq!(b[0], 2);

        b[0] = 5;

        assert_eq!(b[0], 5);
    }

    #[test]
    #[should_panic]
    fn test_index_bigger_than_length() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(2);

        b[2];
    }

    #[test]
    fn test_peek_some() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(1);
        b.push(2);

        assert_eq!(b.peek(), Some(&1));
    }

    #[test]
    fn test_peek_none() {
        let mut b = AllocRingBuffer::with_capacity(2);
        b.push(1);

        assert_eq!(b.peek(), None);
    }
}
