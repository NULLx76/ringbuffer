use core::ops::{Index, IndexMut};

use crate::ringbuffer_trait::RingBuffer;

extern crate alloc;
// We need vecs so depend on alloc
use crate::{ReadableRingbuffer, RingBufferExt, WritableRingbuffer};
use alloc::vec::Vec;
use core::iter::FromIterator;
use core::mem::MaybeUninit;

/// The AllocRingBuffer is a RingBuffer which is based on a Vec. This means it allocates at runtime
/// on the heap, and therefore needs the [`alloc`] crate. This struct and therefore the dependency on
/// alloc can be disabled by disabling the `alloc` (default) feature.
///
/// # Example
/// ```
/// use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt, ReadableRingbuffer};
///
/// let mut buffer = AllocRingBuffer::with_capacity(2);
///
/// // First entry of the buffer is now 5.
/// buffer.push(5).unwrap();
///
/// // The last item we pushed is 5
/// assert_eq!(buffer.front(), Some(&5));
///
/// // Second entry is now 42.
/// buffer.push(42).unwrap();
///
/// assert_eq!(buffer.back(), Some(&5));
/// assert!(buffer.is_full());
///
/// // Because capacity is reached the next push will be the first item of the buffer.
/// buffer.push_force(1);
/// assert_eq!(buffer.to_vec(), vec![42, 1]);
/// ```
#[derive(Debug)]
pub struct AllocRingBuffer<T> {
    buf: Vec<MaybeUninit<T>>,
    capacity: usize,
    readptr: usize,
    writeptr: usize,
}

// We need to manually implement PartialEq because MaybeUninit isn't PartialEq
impl<T: 'static + PartialEq> PartialEq for AllocRingBuffer<T> {
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

impl<T: 'static + PartialEq> Eq for AllocRingBuffer<T> {}

/// The capacity of a RingBuffer created by new or default (`1024`).
// must be a power of 2
pub const RINGBUFFER_DEFAULT_CAPACITY: usize = 1024;

impl<T: 'static> RingBuffer<T> for AllocRingBuffer<T> {
    #[inline]
    fn capacity(&self) -> usize {
        self.capacity
    }

    impl_ringbuffer!(readptr, writeptr);
}

impl<T: 'static> ReadableRingbuffer<T> for AllocRingBuffer<T> {
    #[inline]
    fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            let index = crate::mask(self, self.readptr);
            let res = core::mem::replace(&mut self.buf[index], MaybeUninit::uninit());
            self.readptr += 1;

            // Safety: Because the index must be in bounds for the array, we know this element is
            //       : already initialized.
            Some(unsafe { res.assume_init() })
        } else {
            None
        }
    }

    impl_read_ringbuffer!(readptr);
}

impl<T: 'static> WritableRingbuffer<T> for AllocRingBuffer<T> {
    fn push(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            Err(item)
        } else {
            let index = crate::mask(self, self.writeptr);

            if index >= self.buf.len() {
                self.buf.push(MaybeUninit::new(item));
            } else {
                self.buf[index] = MaybeUninit::new(item);
            }

            self.writeptr += 1;

            Ok(())
        }
    }
}

impl<T: 'static> RingBufferExt<T> for AllocRingBuffer<T> {
    #[inline]
    fn push_force(&mut self, value: T) {
        if self.is_full() {
            let index = crate::mask(self, self.readptr);
            unsafe {
                // make sure we drop whatever is being overwritten
                // SAFETY: the buffer is full, so this must be inited
                //       : also, index has been masked
                // make sure we drop because it won't happen automatically
                core::ptr::drop_in_place(self.buf[index].as_mut_ptr());
            }
            self.readptr += 1;
        }

        let index = crate::mask(self, self.writeptr);

        if index >= self.buf.len() {
            self.buf.push(MaybeUninit::new(value));
        } else {
            self.buf[index] = MaybeUninit::new(value);
        }

        self.writeptr += 1;
    }

    impl_ringbuffer_ext!(
        get_unchecked,
        get_unchecked_mut,
        readptr,
        writeptr,
        crate::mask
    );
}

impl<T> AllocRingBuffer<T> {
    /// Creates a RingBuffer with a certain capacity. This capacity is fixed.
    /// for this ringbuffer to work, cap must be a power of two and greater than zero.
    #[inline]
    pub fn with_capacity_unchecked(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
            capacity: cap,
            readptr: 0,
            writeptr: 0,
        }
    }

    /// Creates a RingBuffer with a certain capacity. The actual capacity is the input to the
    /// function raised to the power of two (effectively the input is the log2 of the actual capacity)
    #[inline]
    pub fn with_capacity_power_of_2(cap_power_of_two: usize) -> Self {
        Self::with_capacity_unchecked(cap_power_of_two.pow(2))
    }

    #[inline]
    /// Creates a RingBuffer with a certain capacity. The capacity must be a power of two.
    pub fn with_capacity(cap: usize) -> Self {
        assert_ne!(cap, 0, "Capacity must be greater than 0");
        assert!(cap.is_power_of_two(), "Capacity must be a power of two");

        Self::with_capacity_unchecked(cap)
    }

    /// Creates a RingBuffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference from the buffer without checking it is initialized.
    /// Caller must be sure the index is in bounds, or this will panic.
    /// However, it's not unsafe -- only unsafe to match signature of other methods.
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.buf[index]
            .as_ptr()
            .as_ref()
            .expect("const array ptr shouldn't be null!")
    }

    /// Get a mut reference from the buffer without checking it is initialized.
    /// Caller must be sure the index is in bounds, or this will panic.
    /// However, it's not unsafe -- only unsafe to match signature of other methods.
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.buf[index]
            .as_mut_ptr()
            .as_mut()
            .expect("const array ptr shouldn't be null!")
    }
}

impl<RB: 'static> FromIterator<RB> for AllocRingBuffer<RB> {
    fn from_iter<T: IntoIterator<Item = RB>>(iter: T) -> Self {
        let mut res = Self::default();
        for i in iter {
            res.push_force(i)
        }

        res
    }
}

impl<T> Default for AllocRingBuffer<T> {
    /// Creates a buffer with a capacity of [crate::RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    fn default() -> Self {
        let cap = RINGBUFFER_DEFAULT_CAPACITY;
        Self {
            buf: Vec::with_capacity(cap),
            capacity: cap,
            readptr: 0,
            writeptr: 0,
        }
    }
}

impl<T: 'static> Index<usize> for AllocRingBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T: 'static> IndexMut<usize> for AllocRingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::alloc::vec::Vec;
    use crate::{AllocRingBuffer, RingBuffer, RingBufferExt, RINGBUFFER_DEFAULT_CAPACITY};

    #[test]
    fn test_default() {
        let b: AllocRingBuffer<u32> = AllocRingBuffer::default();
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.capacity());
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.buf.capacity());
        assert_eq!(b.capacity, b.capacity());
        assert_eq!(b.buf.len(), b.len());
        assert_eq!(0, b.writeptr);
        assert_eq!(0, b.readptr);
        assert!(b.is_empty());
        assert!(b.buf.is_empty());
        assert_eq!(0, b.iter().count());

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
    fn test_with_capacity_power_of_two() {
        let b = AllocRingBuffer::<i32>::with_capacity_power_of_2(2);
        assert_eq!(b.capacity, 4);
    }

    #[test]
    #[should_panic]
    fn test_with_capacity_no_power_of_two() {
        let _ = AllocRingBuffer::<i32>::with_capacity(10);
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = AllocRingBuffer::<i32>::with_capacity(2);
        let _ = b[2];
    }
}
