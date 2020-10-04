use crate::RingBuffer;
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};

/// The RingBuffer struct.
///
/// # Example
/// ```
/// use ringbuffer::ConstGenericRingBuffer;
/// use ringbuffer::RingBuffer;
///
/// let mut buffer = ConstGenericRingBuffer::<_, 2>::new();
///
/// // First entry of the buffer is now 5.
/// buffer.push(5);
///
/// assert_eq!(buffer[-1], 5);
///
/// // Second entry is now 42.
/// buffer.push(42);
///
/// // Because capacity is reached the next push will be the first item of the buffer.
/// buffer.push(1);
/// assert_eq!(buffer[-1], 1);
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct ConstGenericRingBuffer<T, const CAP: usize> {
    buf: [T; CAP],
    index: usize,
    length_counter: usize,
}

/// It is only possible to create a Generic RingBuffer if the type T in it implements Default.
/// This is because the array needs to be allocated at compile time, and needs to be filled with
/// some default value to avoid unsafe.
impl<T: Default, const CAP: usize> ConstGenericRingBuffer<T, CAP> {
    /// Creates a new RingBuffer. The method is here for compatibility with the alloc version of
    /// RingBuffer. This method simply creates a default ringbuffer. The capacity is given as a
    /// type parameter.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, const CAP: usize> ConstGenericRingBuffer<T, CAP> {
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
}

impl<T: 'static + Default, const CAP: usize> RingBuffer<T> for ConstGenericRingBuffer<T, CAP> {
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
        CAP
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

impl<T: Default, const CAP: usize> Default for ConstGenericRingBuffer<T, CAP> {
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

impl<T: 'static + Default, const CAP: usize> Index<isize> for ConstGenericRingBuffer<T, CAP> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T: 'static + Default, const CAP: usize> IndexMut<isize> for ConstGenericRingBuffer<T, CAP> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_no_empty() {
        ConstGenericRingBuffer::<u32, 0>::new();
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = ConstGenericRingBuffer::<i32, 2>::new();
        b[2];
    }

    #[test]
    fn test_uninit() {
        let mut b = unsafe { ConstGenericRingBuffer::<_, 2>::new_uninit() };
        assert_eq!(b.peek(), None);

        assert_eq!(b.len(), 0);
        assert_eq!(b.capacity(), 2);

        b.push(1);
        b.push(2);
        b.push(3);

        assert_eq!(b.len(), 2);
        assert_eq!(b.capacity(), 2);

        assert_eq!(b.get_absolute(0).unwrap(), &3);
        assert_eq!(b.get_absolute(1).unwrap(), &2);
    }

    #[test]
    #[should_panic]
    fn test_uninit_out_of_bounds() {
        let b = unsafe { ConstGenericRingBuffer::<i32, 2>::new_uninit() };
        b[0];
    }
}
