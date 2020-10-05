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
    cap: usize,
    readptr: usize,
    writeptr: usize,
}

/// It is only possible to create a Generic RingBuffer if the type T in it implements Default.
/// This is because the array needs to be allocated at compile time, and needs to be filled with
/// some default value.
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
        assert_ne!(Cap::to_usize(), 0, "Capacity must be greater than 0");
        assert!(
            Cap::to_usize().is_power_of_two(),
            "Capacity must be a power of two"
        );

        Self {
            buf: GenericArray::default(),
            cap: Cap::to_usize(),
            readptr: 0,
            writeptr: 0,
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
    #[inline(always)]
    #[cfg(not(tarpaulin_include))]
    fn capacity(&self) -> usize {
        self.cap
    }

    fn push(&mut self, value: T) {
        if self.is_full() {
            self.readptr += 1;
        }
        let index = crate::mask(self, self.writeptr);
        self.buf[index] = value;
        self.writeptr += 1;
    }

    impl_ringbuffer!(buf, readptr, writeptr, crate::mask);
}

#[cfg(test)]
mod tests {
    use crate::GenericRingBuffer;
    use generic_array::typenum;

    #[test]
    #[should_panic]
    fn test_no_empty() {
        let _ = GenericRingBuffer::<u32, typenum::U0>::new();
    }

    #[test]
    #[should_panic]
    fn test_with_capacity_no_power_of_two() {
        let _ = GenericRingBuffer::<i32, typenum::U10>::new();
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = GenericRingBuffer::<i32, typenum::U2>::new();
        let _ = b[2];
    }
}
