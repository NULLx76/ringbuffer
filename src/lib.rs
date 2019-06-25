use std::iter::Chain;
use std::slice::Iter as SliceIter;

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

pub type Iter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;

const RINGBUFFER_DEFAULT_CAPACITY: usize = 1024;

impl<T> RingBuffer<T> {
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "Capacity must be greater than zero");

        Self {
            buf: Vec::with_capacity(cap),
            cap,
            index: 0,
        }
    }

    #[inline]
    pub fn new() -> Self {
        RingBuffer::default()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buf.clear();
        self.index = 0;
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn push(&mut self, e: T) {
        if self.buf.len() < self.capacity() {
            self.buf.push(e);
        } else {
            self.buf[self.index] = e;
        }

        self.index = (self.index + 1) % self.capacity()
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        let (l, r) = self.buf.split_at(self.index);
        r.iter().chain(l.iter())
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Copy,
    {
        self.iter().map(|&e| e).collect()
    }
    // TODO:
    //    pub fn iter_mut(&self) -> IterMut<T> {
    //        unimplemented!();
    //    }
}

impl<T> Default for RingBuffer<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
