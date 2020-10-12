use crate::RingBuffer;
use core::iter::FromIterator;
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};

/// The ConstGenericRingBuffer struct is a RingBuffer implementation which does not require `alloc`.
/// However, it does require the still unstable rust feature `const-generics`. Therefore this struct
/// is feature-gated behind the `const_generics` feature and when enabled only works on nightly rust.
///
/// [`ConstGenericRingBuffer`] allocates the ringbuffer on the stack, and the size must be known at
/// compile time through const-generics.
///
/// # Example
/// ```
/// use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
///
/// let mut buffer = ConstGenericRingBuffer::<_, 2>::new();
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
#[derive(Debug)]
pub struct ConstGenericRingBuffer<T, const CAP: usize> {
    buf: [MaybeUninit<T>; CAP],
    readptr: usize,
    writeptr: usize,
}

// We need to manually implement PartialEq because MaybeUninit isn't PartialEq
impl<T: 'static + PartialEq, const CAP: usize> PartialEq for ConstGenericRingBuffer<T, CAP> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for (a, b) in self.iter().zip(other.iter()) {
                if a != b {
                    return false;
                }
            }
            true
        }
    }
}

impl<T: 'static + PartialEq, const CAP: usize> Eq for ConstGenericRingBuffer<T, CAP> {}

impl<T, const CAP: usize> ConstGenericRingBuffer<T, CAP> {
    /// Creates a new RingBuffer. The method is here for compatibility with the alloc version of
    /// RingBuffer. This method simply creates a default ringbuffer. The capacity is given as a
    /// type parameter.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference from the buffer without checking it is initialized
    /// Caller MUST be sure this index is initialized, or undefined behavior will happen
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.buf[index]
            .as_ptr()
            .as_ref()
            .expect("const array ptr shouldn't be null!")
    }

    /// Get a mutable reference from the buffer without checking it is initialized
    /// Caller MUST be sure this index is initialized, or undefined behavior will happen
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.buf[index]
            .as_mut_ptr()
            .as_mut()
            .expect("const array ptr shouldn't be null!")
    }
}

impl<T: 'static, const CAP: usize> RingBuffer<T> for ConstGenericRingBuffer<T, CAP> {
    #[inline]
    #[cfg(not(tarpaulin_include))]
    fn capacity(&self) -> usize {
        CAP
    }

    #[inline]
    fn push(&mut self, value: T) {
        if self.is_full() {
            unsafe {
                // make sure we drop whatever is being overwritten
                // SAFETY: the buffer is full, so this must be inited
                //       : also, index has been masked
                let index = crate::mask(self, self.readptr);
                // make sure we drop because it won't happen automatically
                core::ptr::drop_in_place(self.buf[index].as_mut_ptr());
            }
            self.readptr += 1;
        }
        let index = crate::mask(self, self.writeptr);
        self.buf[index] = MaybeUninit::new(value);
        self.writeptr += 1;
    }

    #[inline]
    fn dequeue_ref(&mut self) -> Option<&T> {
        if !self.is_empty() {
            let index = crate::mask(self, self.readptr);
            self.readptr += 1;
            let res = unsafe {
                // SAFETY: index has been masked
                self.get_unchecked(index)
            };

            Some(res)
        } else {
            None
        }
    }

    impl_ringbuffer!(readptr, writeptr, crate::mask);
}

impl<T, const CAP: usize> Default for ConstGenericRingBuffer<T, CAP> {
    /// Creates a buffer with a capacity specified through the Cap type parameter.
    #[inline]
    fn default() -> Self {
        assert_ne!(CAP, 0, "Capacity must be greater than 0");
        assert!(CAP.is_power_of_two(), "Capacity must be a power of two");

        let arr = array_init::array_init(|_| MaybeUninit::uninit());

        Self {
            buf: arr,
            writeptr: 0,
            readptr: 0,
        }
    }
}

impl<RB: 'static, const CAP: usize> FromIterator<RB> for ConstGenericRingBuffer<RB, CAP> {
    fn from_iter<T: IntoIterator<Item = RB>>(iter: T) -> Self {
        let mut res = Self::default();
        for i in iter {
            res.push(i)
        }

        res
    }
}

impl<T: 'static, const CAP: usize> Index<isize> for ConstGenericRingBuffer<T, CAP> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T: 'static, const CAP: usize> IndexMut<isize> for ConstGenericRingBuffer<T, CAP> {
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
        let _ = ConstGenericRingBuffer::<u32, 0>::new();
    }

    #[test]
    #[should_panic]
    fn test_with_capacity_no_power_of_two() {
        let _ = ConstGenericRingBuffer::<i32, 10>::new();
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = ConstGenericRingBuffer::<i32, 2>::new();
        let _ = b[2];
    }
}
