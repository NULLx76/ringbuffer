use crate::ringbuffer_trait::RingBuffer;

extern crate alloc;
// We need vecs so depend on alloc
use crate::{ReadableRingbuffer, WritableRingbuffer};
use alloc::vec::Vec;

/// The AllocRingBuffer is a RingBuffer which is based on a Vec. This means it allocates at runtime
/// on the heap, and therefore needs the [`alloc`] crate. This struct and therefore the dependency on
/// alloc can be disabled by disabling the `alloc` (default) feature.
///
/// # Example
/// ```rust
/// // TODO: Example
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct ThreadAllocRingBuffer<T> {
    buf: Vec<T>,
    capacity: usize,

    readptr: usize,
    writeptr: usize,
}

/// The capacity of a RingBuffer created by new or default (`1024`).
// must be a power of 2
pub const RINGBUFFER_DEFAULT_CAPACITY: usize = 1024;

impl<T: 'static + Default> RingBuffer<T> for ThreadAllocRingBuffer<T> {
    #[inline]
    fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    fn len(&self) -> usize {
        self.writeptr - self.readptr
    }

    #[inline]
    fn clear(&mut self) {
        self.readptr = 0;
    }
}

impl<T: 'static + Default> ReadableRingbuffer<T> for ThreadAllocRingBuffer<T> {
    #[inline]
    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    impl_read_ringbuffer!(buf, readptr, writeptr, crate::mask);
}

impl<T: 'static + Default> WritableRingbuffer<T> for ThreadAllocRingBuffer<T> {
    type PushError = T;

    fn push(&mut self, _item: T) -> Result<(), Self::PushError> {
        todo!()

        // if self.is_full() {
        //     Err(item)
        // } else {
        //
        //     let index = crate::mask(self, writeptr);
        //
        //     if index >= self.buf.len() {
        //         self.buf.push(item);
        //     } else {
        //         self.buf[index] = item;
        //     }
        //
        //     let _ = self.writeptr.fetch_add(1, Ordering::SeqCst);
        //
        //     Ok(())
        // }
    }
}

impl<T> ThreadAllocRingBuffer<T> {
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

    /// Creates a RingBuffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY](crate::RINGBUFFER_DEFAULT_CAPACITY).
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for ThreadAllocRingBuffer<T> {
    /// Creates a buffer with a capacity of [crate::RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    fn default() -> Self {
        let cap = RINGBUFFER_DEFAULT_CAPACITY;
        Self {
            buf: Vec::with_capacity(cap),
            capacity: cap,

            writeptr: 0,
            readptr: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::alloc::vec::Vec;
    use crate::{RingBuffer, ThreadAllocRingBuffer, RINGBUFFER_DEFAULT_CAPACITY};

    #[test]
    fn test_default() {
        let b: ThreadAllocRingBuffer<u32> = ThreadAllocRingBuffer::default();
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.capacity());
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.buf.capacity());
        assert_eq!(b.capacity, b.capacity());
        assert_eq!(b.buf.len(), b.len());
        assert_eq!(0, b.writeptr);
        assert_eq!(0, b.readptr);
        assert!(b.is_empty());
        assert!(b.buf.is_empty());
        assert_eq!(
            Vec::<u32>::with_capacity(RINGBUFFER_DEFAULT_CAPACITY),
            b.buf
        );
    }

    #[test]
    fn test_default_capacity_constant() {
        // This is to prevent accidentally changing it.
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, 1024)
    }

    #[test]
    fn test_with_capacity_power_of_two() {
        let b = ThreadAllocRingBuffer::<i32>::with_capacity_power_of_2(2);
        assert_eq!(b.capacity, 4);
    }

    #[test]
    #[should_panic]
    fn test_with_capacity_no_power_of_two() {
        let _ = ThreadAllocRingBuffer::<i32>::with_capacity(10);
    }
}
