use core::ops::{Index, IndexMut};

use generic_array::{GenericArray, ArrayLength};
pub use generic_array::typenum;
use core::marker::PhantomData;


#[cfg(feature="alloc")]
extern crate alloc;
#[cfg(feature="alloc")]
use alloc::vec::Vec;

/// The RingBuffer struct.
///
/// # Example
/// ```
/// use ringbuffer::GenericRingBuffer;
/// use ringbuffer::typenum;
///
/// let mut buffer = GenericRingBuffer::<_, typenum::U2>::new();
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
#[derive(PartialEq,Eq,Debug)]
pub struct RingBuffer<T, Cap: ArrayLength<T>> {
    buf: GenericArray<T, Cap>,
    index: usize,
    length_counter: usize,
}

/// It is only possible to create a Generic RingBuffer if the type T in it implements Default.
/// This is because the array needs to be allocated at compile time, and needs to be filled with
/// some default value to avoid unsafe.
impl<T: Default, Cap: ArrayLength<T>> RingBuffer<T, Cap> {
    /// Creates a new RingBuffer. The method is here for compatibility with the alloc version of
    /// RingBuffer. This method simply creates a default ringbuffer. The capacity is given as a
    /// type parameter.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct UninitExactIter<T, Cap> {
    count: usize,
    phantom1: PhantomData<T>,
    phantom2: PhantomData<Cap>,
}

impl<T, Cap: ArrayLength<T>> Default for UninitExactIter<T, Cap> {
    fn default() -> Self {
        Self {
            count: 0,
            phantom1: Default::default(),
            phantom2: Default::default()
        }
    }
}

impl<T, Cap: ArrayLength<T>> Iterator for UninitExactIter<T, Cap> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.count += 1;

        if self.count <= Cap::to_usize() {
            let elem = unsafe{
                core::mem::MaybeUninit::<T>::uninit().assume_init()
            };

            Some(elem)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (Cap::to_usize() - self.count, Some(Cap::to_usize() - self.count))
    }
}

impl<T, Cap: ArrayLength<T>> ExactSizeIterator for UninitExactIter<T, Cap> {
    fn len(&self) -> usize {
        Cap::to_usize()
    }
}


impl<T, Cap: ArrayLength<T>> RingBuffer<T, Cap> {
    /// Creates a new RingBuffer with uninitialized elements. This is unsafe because this relies on
    /// creating uninitialized memory. However, it is not inherently unsafe. The implementation makes
    /// sure no uninitialized memory can *ever* be accessed through the RingBuffer struct.
    ///
    /// Still it's recommended to use the `new`, `default` or `with_capacity` methods to create a
    /// RingBuffer, whenever the type T implements default.
    #[inline]
    #[cfg(feature="generic_uninit")]
    pub unsafe fn new_uninit() -> Self {
        Self {
            buf: GenericArray::from_exact_iter(UninitExactIter::<T, Cap>::default())
                .expect("UninitExactIter was made with Cap so must be the same size as the generic array."),
            index: 0,
            length_counter: 0,
        }
    }

    /// Returns the length of the internal buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.length_counter
    }

    /// Returns true if the buffer is empty, some value between 0 and capacity.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length_counter == 0
    }

    /// Empties the buffer.
    #[inline]
    pub fn clear(&mut self) {
        self.index = 0;
        self.length_counter = 0;
    }

    /// Returns the capacity of the buffer.
    #[inline]
    pub fn capacity(&self) -> usize {
        Cap::to_usize()
    }

    /// Pushes a value onto the buffer. Cycles around if capacity is reached.
    pub fn push(&mut self, e: T) {
        self.buf[self.index] = e;
        if self.length_counter < self.capacity() {
            self.length_counter += 1
        }
        self.index = (self.index + 1) % self.capacity()
    }

    /// Returns the value at the current index.
    /// This is the value that will be overwritten by the next push.
    pub fn peek(&self) -> Option<&T> {
        if self.index >= self.len() {
            None
        } else {
            self.buf.get(self.index)
        }
    }

    /// Creates an iterator over the buffer starting from the latest push.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (l, r) = self.buf[0..self.length_counter].split_at(self.index);
        r.iter().chain(l.iter())
    }

    ///  Creates a mutable iterator over the buffer starting from the latest push.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let (l, r) = self.buf[0..self.length_counter].split_at_mut(self.index);
        r.iter_mut().chain(l.iter_mut())
    }

    /// Converts the buffer to an vector.
    #[inline]
    #[cfg(feature="alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<T>
        where
            T: Copy,
    {
        self.iter().copied().collect()
    }
}

impl<T: Default, Cap: ArrayLength<T>> Default for RingBuffer<T, Cap> {

    /// Creates a buffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    fn default() -> Self {
        assert_ne!(Cap::to_usize(), 0);

        Self {
            buf: GenericArray::default(),
            index: 0,
            length_counter: 0
        }
    }
}

impl<T, Cap: ArrayLength<T>> Index<usize> for RingBuffer<T, Cap> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T, Cap: ArrayLength<T>> IndexMut<usize> for RingBuffer<T, Cap> {
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
        let b: RingBuffer<i32, typenum::U10> = RingBuffer::default();
        assert_eq!(b.capacity(), 10);
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn test_new() {
        let b: RingBuffer<i32, typenum::U10> = RingBuffer::new();
        assert_eq!(b.capacity(), 10);
        assert_eq!(b.len(), 0);
    }


    #[test]
    fn test_default_eq_new() {
        assert_eq!(RingBuffer::<u32, typenum::U10>::default(), RingBuffer::<u32, typenum::U10>::new())
    }

    #[test]
    #[should_panic]
    fn test_no_empty() {
        RingBuffer::<u32, typenum::U0>::new();
    }

    #[test]
    fn test_len() {
        let mut b = RingBuffer::<_, typenum::U10>::new();
        assert_eq!(0, b.len());
        b.push(1);
        assert_eq!(1, b.len());
        b.push(2);
        assert_eq!(2, b.len())
    }

    #[test]
    fn test_len_wrap() {
        let mut b = RingBuffer::<_, typenum::U2>::new();
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
        let mut b = RingBuffer::<_, typenum::U10>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        b.clear();
        assert!(b.is_empty());
        assert_eq!(0, b.len());
    }

    #[test]
    fn test_empty() {
        let mut b = RingBuffer::<_, typenum::U10>::new();
        assert!(b.is_empty());
        b.push(1);
        b.push(2);
        b.push(3);
        assert_ne!(b.is_empty(), true);

        b.clear();
        assert!(b.is_empty());
        assert_eq!(0, b.len());
    }

    #[test]
    fn test_iter() {
        let mut b = RingBuffer::<_, typenum::U10>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        let mut iter = b.iter();
        assert_eq!(&1, iter.next().unwrap());
        assert_eq!(&2, iter.next().unwrap());
        assert_eq!(&3, iter.next().unwrap());
    }

    #[test]
    fn test_iter_wrap() {
        let mut b = RingBuffer::<_, typenum::U2>::new();
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
        let mut b = RingBuffer::<_, typenum::U10>::new();
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
        let mut b = RingBuffer::<_, typenum::U2>::new();
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
        let mut b = RingBuffer::<_, typenum::U10>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        assert_eq!(vec![1,2,3], b.to_vec())
    }

    #[test]
    fn test_to_vec_wrap() {
        let mut b = RingBuffer::<_, typenum::U2>::new();
        b.push(1);
        b.push(2);
        // Wrap
        b.push(3);

        assert_eq!(vec![2,3], b.to_vec())
    }

    #[test]
    fn test_index() {
        let mut b = RingBuffer::<_, typenum::U10>::new();
        b.push(2);

        assert_eq!(b[0], 2)
    }

    #[test]
    fn test_index_mut() {
        let mut b = RingBuffer::<_, typenum::U10>::new();
        b.push(2);

        assert_eq!(b[0], 2);

        b[0] = 5;

        assert_eq!(b[0], 5);
    }

    #[test]
    #[should_panic]
    fn test_index_bigger_than_length() {
        let mut b = RingBuffer::<_, typenum::U2>::new();
        b.push(2);

        b[2];
    }

    #[test]
    fn test_peek_some() {
        let mut b = RingBuffer::<_, typenum::U2>::new();
        b.push(1);
        b.push(2);

        assert_eq!(b.peek(),Some(&1));
    }

    #[test]
    fn test_peek_none() {
        let mut b = RingBuffer::<_, typenum::U10>::new();
        b.push(1);

        assert_eq!(b.peek(),None);
    }

    #[test]
    fn test_uninit() {
        let mut b = unsafe { RingBuffer::<_, typenum::U2>::new_uninit() };
        assert_eq!(b.peek(), None);

        assert_eq!(b.len(), 0);
        assert_eq!(b.capacity(), 2);

        b.push(1);
        b.push(2);
        b.push(3);

        assert_eq!(b.len(), 2);
        assert_eq!(b.capacity(), 2);

        assert_eq!(b[0], 3);
        assert_eq!(b[1], 2);
    }

    #[test]
    fn test_just_to_have_100_percent_coverage() {
        let mut u = UninitExactIter::<i32, typenum::U2>::default();
        assert_eq!(u.size_hint(), (2, Some(2)));
        assert_eq!(u.len(), 2);
        assert!(u.next().is_some());
        assert_eq!(u.size_hint(), (1, Some(1)));
        assert!(u.next().is_some());
        assert_eq!(u.size_hint(), (0, Some(0)));
        assert!(u.next().is_none());
    }
}
