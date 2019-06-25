use std::iter::{Chain, Rev};
use std::slice::Iter as SliceIter;

pub struct RingBuffer<T> {
    buf: Vec<T>,
    capacity: usize,
    index: usize,
}

pub type Iter<'a, T> = Chain<Rev<SliceIter<'a, T>>, Rev<SliceIter<'a, T>>>;

impl<T> RingBuffer<T> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity < 1 {
            panic!("Capacity needs to be at least 1")
        }

        Self {
            buf: Vec::with_capacity(capacity),
            capacity,
            index: 0,
        }
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
        self.capacity
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
        l.iter().rev().chain(r.iter().rev())
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
    fn default() -> Self {
        Self {
            buf: Vec::with_capacity(4096),
            capacity: 4096,
            index: 0,
        }
    }
}

// TODO: Some tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
