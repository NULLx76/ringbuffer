use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// The RingBuffer struct.
///
/// # Example
/// ```
/// use ringbuffer::ConstGenericRingBuffer;
///
/// let mut buffer = ConstGenericRingBuffer::<_, 2>::new();
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
pub struct RingBuffer<T, const CAP: usize> {
    buf: [T; CAP],
    index: usize,
    length_counter: usize,
}

/// It is only possible to create a Generic RingBuffer if the type T in it implements Default.
/// This is because the array needs to be allocated at compile time, and needs to be filled with
/// some default value to avoid unsafe.
impl<T: Default, const CAP: usize> RingBuffer<T, CAP> {
    /// Creates a new RingBuffer. The method is here for compatibility with the alloc version of
    /// RingBuffer. This method simply creates a default ringbuffer. The capacity is given as a
    /// type parameter.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, const CAP: usize> RingBuffer<T, CAP> {
    /// Creates a new RingBuffer with uninitialized elements. This is unsafe because this relies on
    /// creating uninitialized memory.
    ///
    /// Still it's recommended to use the `new`, `default` or `with_capacity` methods to create a
    /// RingBuffer, whenever the type T implements default.
    ///
    /// # Safety
    ///
    /// Using this function is not actually that unsafe, because the ringbuffer makes sure you have
    /// to write to an unitialized element before you can read it by only moving the index forward
    /// when you write. Therefore you can't ever accidentally read uninitialized memory.
    #[inline]
    #[cfg(feature = "generic_uninit")]
    pub unsafe fn new_uninit() -> Self {
        let arr: [T; CAP] = MaybeUninit::uninit().assume_init();

        Self {
            buf: arr,
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
    #[cfg(not(tarpaulin_include))]
    pub fn capacity(&self) -> usize {
        CAP
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
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Copy,
    {
        self.iter().copied().collect()
    }
}

impl<T: Default, const CAP: usize> Default for RingBuffer<T, CAP> {
    /// Creates a buffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    fn default() -> Self {
        assert_ne!(CAP, 0);

        // Requires unsafe block because currently it is impossible to create a const generic array
        // from Default elements that are not copy. All elements are initialized below and thus
        // it is impossible to actually access unitialized memory. Even if elements weren't initialized
        // (like with the new_uninit constructor), the RingBuffer makes sure it's never possible
        // to access elements that are not initialized.
        let mut arr: [T; CAP] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in &mut arr {
            *i = T::default()
        }

        Self {
            buf: arr,
            index: 0,
            length_counter: 0,
        }
    }
}

impl<T, const CAP: usize> Index<usize> for RingBuffer<T, CAP> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.length_counter);
        &self.buf[index]
    }
}

impl<T, const CAP: usize> IndexMut<usize> for RingBuffer<T, CAP> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.length_counter);
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
        let b: RingBuffer<i32, 10> = RingBuffer::default();
        assert_eq!(b.capacity(), 10);
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn test_new() {
        let b: RingBuffer<i32, 10> = RingBuffer::new();
        assert_eq!(b.capacity(), 10);
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn test_default_eq_new() {
        assert_eq!(
            RingBuffer::<u32, 10>::default(),
            RingBuffer::<u32, 10>::new()
        )
    }

    #[test]
    #[should_panic]
    fn test_no_empty() {
        RingBuffer::<u32, 0>::new();
    }

    #[test]
    fn test_len() {
        let mut b = RingBuffer::<_, 10>::new();
        assert_eq!(0, b.len());
        b.push(1);
        assert_eq!(1, b.len());
        b.push(2);
        assert_eq!(2, b.len())
    }

    #[test]
    fn test_len_wrap() {
        let mut b = RingBuffer::<_, 2>::new();
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
        let mut b = RingBuffer::<_, 10>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        b.clear();
        assert!(b.is_empty());
        assert_eq!(0, b.len());
    }

    #[test]
    fn test_empty() {
        let mut b = RingBuffer::<_, 10>::new();
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
        let mut b = RingBuffer::<_, 10>::new();
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
        let mut b = RingBuffer::<_, 2>::new();
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
        let mut b = RingBuffer::<_, 10>::new();
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
        let mut b = RingBuffer::<_, 2>::new();
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
        let mut b = RingBuffer::<_, 10>::new();
        b.push(1);
        b.push(2);
        b.push(3);

        assert_eq!(vec![1, 2, 3], b.to_vec())
    }

    #[test]
    fn test_to_vec_wrap() {
        let mut b = RingBuffer::<_, 2>::new();
        b.push(1);
        b.push(2);
        // Wrap
        b.push(3);

        assert_eq!(vec![2, 3], b.to_vec())
    }

    #[test]
    fn test_index() {
        let mut b = RingBuffer::<_, 10>::new();
        b.push(2);

        assert_eq!(b[0], 2)
    }

    #[test]
    fn test_index_mut() {
        let mut b = RingBuffer::<_, 10>::new();
        b.push(2);

        assert_eq!(b[0], 2);

        b[0] = 5;

        assert_eq!(b[0], 5);
    }

    #[test]
    #[should_panic]
    fn test_index_bigger_than_length() {
        let mut b = RingBuffer::<_, 2>::new();
        b.push(2);

        b[2];
    }

    #[test]
    fn test_peek_some() {
        let mut b = RingBuffer::<_, 2>::new();
        b.push(1);
        b.push(2);

        assert_eq!(b.peek(), Some(&1));
    }

    #[test]
    fn test_peek_none() {
        let mut b = RingBuffer::<_, 10>::new();
        b.push(1);

        assert_eq!(b.peek(), None);
    }

    #[test]
    fn test_uninit() {
        let mut b = unsafe { RingBuffer::<_, 2>::new_uninit() };
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
    #[should_panic]
    fn test_uninit_out_of_bounds() {
        let mut b = unsafe { RingBuffer::<i32, 2>::new_uninit() };
        b[0];
    }
}
