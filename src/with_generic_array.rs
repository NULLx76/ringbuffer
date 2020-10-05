use core::ops::{Index, IndexMut};

use crate::RingBuffer;
use core::iter::FromIterator;
use generic_array::{ArrayLength, GenericArray};

/// The GenericRingBuffer struct is a RingBuffer implementation which does not require `alloc`.
/// However it does depend on the typenum crate, to provide compile time integers without needing
/// nightly rust. Unfortunately this is the only way to provide a compile time ringbuffer which
/// does not require nightly rust like `ConstGenericRingBuffer`. Once const-generics are stable
/// this struct will be depricated.
///
/// GenericRingBuffer allocates the ringbuffer on the stack, and the size must be known at
/// compile time through typenum.
///
/// # Example
/// ```
/// use ringbuffer::{RingBuffer, GenericRingBuffer};
/// use ringbuffer::typenum; // for numbers as types in stable rust
///
/// let mut buffer = GenericRingBuffer::<_, typenum::U2>::new();
///
/// // First entry of the buffer is now 5.
/// buffer.push(5);
///
/// // The last item we pushed is 5
/// assert_eq!(buffer.get(-1), Some(&5));
///
/// // Second entry is now 42.
/// buffer.push(42);
///
/// assert_eq!(buffer.peek(), Some(&5));
/// assert!(buffer.is_full());
///
/// // Because capacity is reached the next push will be the first item of the buffer.
/// buffer.push(1);
/// assert_eq!(buffer.to_vec(), vec![42, 1]);
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct GenericRingBuffer<T, Cap: ArrayLength<T>> {
    buf: GenericArray<T, Cap>,
    index: usize,
    length_counter: usize,
}

/// It is only possible to create a Generic RingBuffer if the type T in it implements Default.
/// This is because the array needs to be allocated at compile time, and needs to be filled with
/// some default value to avoid unsafe.
impl<T: Default, Cap: ArrayLength<T>> GenericRingBuffer<T, Cap> {
    /// Creates a new RingBuffer. The method is here for compatibility with the alloc version of
    /// RingBuffer. This method simply creates a default ringbuffer. The capacity is given as a
    /// type parameter.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Default, Cap: ArrayLength<T>> Default for GenericRingBuffer<T, Cap> {
    /// Creates a buffer with a capacity specified through the `Cap` type parameter.
    #[inline]
    fn default() -> Self {
        assert_ne!(Cap::to_usize(), 0);

        Self {
            buf: GenericArray::default(),
            index: 0,
            length_counter: 0,
        }
    }
}

impl<RB: 'static + Default, Cap: ArrayLength<RB>> FromIterator<RB> for GenericRingBuffer<RB, Cap> {
    fn from_iter<T: IntoIterator<Item = RB>>(iter: T) -> Self {
        let mut res = Self::default();
        for i in iter {
            res.push(i)
        }

        res
    }
}

impl<T: 'static + Default, Cap: ArrayLength<T>> Index<isize> for GenericRingBuffer<T, Cap> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T: 'static + Default, Cap: ArrayLength<T>> IndexMut<isize> for GenericRingBuffer<T, Cap> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<T: 'static + Default, Cap: ArrayLength<T>> RingBuffer<T> for GenericRingBuffer<T, Cap> {
    #[inline]
    fn len(&self) -> usize {
        self.length_counter
    }

    #[inline]
    fn clear(&mut self) {
        self.index = 0;
        self.length_counter = 0;
    }

    #[inline]
    #[cfg(not(tarpaulin_include))]
    fn capacity(&self) -> usize {
        Cap::to_usize()
    }

    fn push(&mut self, e: T) {
        self.buf[self.index] = e;
        if self.length_counter < self.capacity() {
            self.length_counter += 1
        }
        self.index = (self.index + 1) % self.capacity()
    }

    impl_ringbuffer!(buf, index);
}

#[cfg(test)]
mod tests {
    use generic_array::typenum;
    use crate::GenericRingBuffer;

    #[test]
    #[should_panic]
    fn test_no_empty() {
        let _ = GenericRingBuffer::<u32, typenum::U0>::new();
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = GenericRingBuffer::<i32, typenum::U2>::new();
        let _ = b[2];
    }
}
