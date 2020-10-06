use core::ops::{Index, IndexMut};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::iter::FromIterator;

// TODO: Remove Default <Issue #13>
/// RingBuffer is a trait defining the standard interface for all RingBuffer
/// implementations ([`AllocRingBuffer`](crate::AllocRingBuffer), [`GenericRingBuffer`](crate::GenericRingBuffer), [`ConstGenericRingBuffer`](crate::ConstGenericRingBuffer))
///
/// This trait only defines methods needed for *any* type of ringbuffer. Methods to actually use the ringbuffer
/// can be found in the [`WritableRingbuffer`], [`ReadableRingBuffer`] and [`MutableRingBuffer`] traits.
///
/// This trait is not object safe, so can't be used dynamically. However it is possible to
/// define a generic function over types implementing RingBuffer.
pub trait RingBuffer<T: 'static + Default>:
    Default + FromIterator<T>
{
    /// Returns the length of the internal buffer.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    ///
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    ///
    /// buffer.push(1);
    /// buffer.push(2);
    /// assert_eq!(buffer.len(), 2);
    ///
    /// buffer.push(3);
    /// assert_eq!(buffer.len(), 2);
    /// ```
    fn len(&self) -> usize;

    /// Returns true if the buffer is entirely empty.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// assert!(buffer.is_empty());
    /// buffer.push(1);
    /// assert!(!buffer.is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true when the length of the ringbuffer equals the capacity. This happens whenever
    /// more elements than capacity have been pushed to the buffer.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(1);
    /// assert!(!buffer.is_full());
    /// buffer.push(1);
    /// assert!(buffer.is_full());
    /// ```
    #[inline]
    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Empties the buffer entirely. Sets the length to 0 but keeps the capacity allocated.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// buffer.clear();
    /// assert!(buffer.is_empty());
    /// ```
    fn clear(&mut self);

    /// Returns the capacity of the buffer.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::<i32>::with_capacity(2);
    /// assert_eq!(buffer.capacity(), 2);
    /// ```
    fn capacity(&self) -> usize;
}

/// Defines RingBuffer methods necessary to write to the ringbuffer in a
pub trait WritableRingbuffer<T: 'static + Default>: RingBuffer<T> {
    /// Pushes a value onto the buffer. Cycles around if capacity is reached.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// buffer.push(2);
    /// buffer.push(3);
    /// assert_eq!(vec![2, 3], buffer.to_vec());
    /// ```
    fn push(&mut self, value: T);

    /// Pushes a value onto the buffer. Returns Err(item) when the buffer is full. Returns Ok(())
    /// when it could push the item.
    // fn try_push(item: T) -> Result<(), T>;

    /// Alias of [`push`](Self::push)
    fn send(&mut self, value: T) {
        self.push(value)
    }
}

/// Defines RingBuffer methods necessary to read from the ringbuffer. This includes dequeue.
pub trait ReadableRingbuffer<T: 'static + Default>: RingBuffer<T> {
    /// Pops the top item off the queue, but does not return it. Instead it is dropped.
    /// If the ringbuffer is empty, this function is a nop.
    fn skip(&mut self);

    /// Dequeues the top item off the ringbuffer and returns an owned version. See the [`pop_ref`](Self::pop_ref) docs
    fn dequeue(&mut self) -> Option<T>;
}

/// Defines Ringbuffer methods necessary to mutate data inside the ringbuffer or query data in the middle
/// of the ringbuffer.
///
/// Notably, the thread safe ringbuffer does *not* implement this trait because
/// to modify or read data in the middle of the buffer would require locking, something we want to avoid.
pub trait RingBufferExt<T: 'static + Default>: RingBuffer<T> + WritableRingbuffer<T> + ReadableRingbuffer<T> + Index<usize, Output = T> + IndexMut<usize> {

    /// Returns true if elem is in the ringbuffer.
    fn contains(&self, elem: &T) -> bool
        where
            T: PartialEq,
    {
        self.iter().any(|i| i == elem)
    }

    /// Returns a reference to the value at the back of the queue.
    /// This is the item that will be dequeued next, and was pushed longest ago.
    #[inline]
    fn back(&self) -> Option<&T> {
        self.get(0)
    }

    /// Returns a mutable reference to the value at the back of the queue.
    /// This is the item that will be dequeued next, and was pushed longest ago.
    #[inline]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Returns a reference to the value at the front of the queue.
    /// This is the item that was last pushed.
    fn front(&self) -> Option<&T>;

    /// Returns a mutable reference to the value at the front of the queue.
    /// This is the item that was last pushed.
    fn front_mut(&mut self) -> Option<&mut T>;

    /// Returns a reference to a value relative to the read end of the ringbuffer.
    /// `get(0)` is the item that will be dequeued next, and is the same as [`back`](Self::back)
    /// `get(1)` is the item that will be dequeued after `get(0)`
    fn get(&self, index: usize) -> Option<&T>;

    /// Returns a mutable reference to a value relative to the read end of the ringbuffer.
    /// `get(0)` is the item that will be dequeued next, and is the same as [`back`](Self::back)
    /// `get(1)` is the item that will be dequeued after `get(0)`
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Gets a value relative to the start of the array (rarely useful, usually you want [`Self::get`])
    fn get_absolute(&self, index: usize) -> Option<&T>;

    /// Gets a value mutably relative to the start of the array (rarely useful, usually you want [`Self::get_mut`])
    fn get_absolute_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Creates a mutable iterator over the buffer starting from the latest push.
    /// Creates a mutable iterator over the buffer starting from the item pushed the longest ago,
    /// and ending at the element most recently pushed.
    #[inline]
    fn iter_mut(&mut self) -> RingBufferMutIterator<T, Self> {
        RingBufferMutIterator::new(self)
    }

    /// Creates an iterator over the buffer starting from the latest push.
    /// Creates an iterator over the buffer starting from the item pushed the longest ago,
    /// and ending at the element most recently pushed.
    #[inline]
    fn iter(&self) -> RingBufferIterator<T, Self> {
        RingBufferIterator::new(self)
    }

    /// Converts the buffer to a vector. This Copies all elements in the ringbuffer.
    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<T>
        where
            T: Clone,
    {
        self.iter().cloned().collect()
    }
}

/// Trait which combines [`ReadableRingbuffer`] and [`WritableRingbuffer`]
pub trait ReadWriteRingbuffer<T: 'static + Default>: RingBuffer<T> + WritableRingbuffer<T> + ReadableRingbuffer<T> {}

impl<S, T: 'static + Default> ReadWriteRingbuffer<T> for S where S: RingBuffer<T> + ReadableRingbuffer<T> + WritableRingbuffer<T> {

}


mod iter {
    use crate::RingBufferExt;
    use core::marker::PhantomData;

    /// RingBufferIterator holds a reference to a RingBuffer and iterates over it. `index` is the
    /// current iterator position.
    pub struct RingBufferIterator<'rb, T: 'static + Default, RB: RingBufferExt<T>> {
        obj: &'rb RB,
        index: usize,
        phantom: PhantomData<T>,
    }

    impl<'rb, T: 'static + Default, RB: RingBufferExt<T>> RingBufferIterator<'rb, T, RB> {
        #[inline]
        pub fn new(obj: &'rb RB) -> Self {
            Self {
                obj,
                index: 0,
                phantom: Default::default(),
            }
        }
    }

    impl<'rb, T: 'static + Default, RB: RingBufferExt<T>> Iterator for RingBufferIterator<'rb, T, RB> {
        type Item = &'rb T;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            let res = self.obj.get(self.index);
            self.index += 1;
            res
        }
    }

    /// RingBufferMutIterator holds a reference to a RingBuffer and iterates over it. `index` is the
    /// current iterator position.
    ///
    /// WARNING: NEVER ACCESS THE `obj` FIELD. it's private on purpose, and can technically be accessed
    /// in the same module. However, this breaks the safety of `next()`
    pub struct RingBufferMutIterator<'rb, T: 'static + Default, RB: RingBufferExt<T>> {
        obj: &'rb mut RB,
        index: usize,
        phantom: PhantomData<T>,
    }

    impl<'rb, T: 'static + Default, RB: RingBufferExt<T>> RingBufferMutIterator<'rb, T, RB> {
        #[inline]
        pub fn new(obj: &'rb mut RB) -> Self {
            Self {
                obj,
                index: 0,
                phantom: Default::default(),
            }
        }

        pub fn next(&mut self) -> Option<&mut T> {
            let res = self.obj.get_mut(self.index);
            self.index += 1;

            res
        }
    }
}

pub use iter::{RingBufferIterator, RingBufferMutIterator};

/// Implement the get, get_mut, get_absolute and get_absolute_mut functions on implementors
/// of RingBuffer. This is to avoid duplicate code.
macro_rules! impl_ringbuffer {
    ($buf: ident, $readptr: ident, $writeptr: ident, $mask: expr) => {
        #[inline]
        fn len(&self) -> usize {
            self.$writeptr - self.$readptr
        }

        #[inline]
        fn clear(&mut self) {
            self.$readptr = 0;
            self.$writeptr = 0;
        }
    };
}

macro_rules! impl_read_ringbuffer {
    ($buf: ident, $readptr: ident, $writeptr: ident, $mask: expr) => {
        #[inline]
        fn skip(&mut self) {
            self.readptr += 1;
        }
    }
}

macro_rules! impl_ringbuffer_ext {
    ($buf: ident, $readptr: ident, $writeptr: ident, $mask: expr) => {
        #[inline]
        fn get(&self, index: usize) -> Option<&T> {
            if self.is_empty() || index >= self.len() {
                None
            } else {
                let masked_index = $mask(self, self.$readptr + index);
                self.$buf.get(masked_index)
            }
        }

        #[inline]
        fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            if self.is_empty() || index >= self.len() {
                None
            } else {
                let masked_index = $mask(self, self.$readptr + index);
                self.$buf.get_mut(masked_index)
            }
        }

        #[inline]
        fn get_absolute(&self, index: usize) -> Option<&T> {
            let read = $mask(self, self.$readptr);
            let write = $mask(self, self.$writeptr);
            if index >= read && index < write {
                self.$buf.get(index)
            } else {
                None
            }
        }

        #[inline]
        fn get_absolute_mut(&mut self, index: usize) -> Option<&mut T> {
            if index >= $mask(self, self.$readptr) && index < $mask(self, self.$writeptr) {
                self.$buf.get_mut(index)
            } else {
                None
            }
        }

        #[inline]
        fn front(&self) -> Option<&T> {
            if !self.is_empty() {
                let masked_index = $mask(self, self.$writeptr - 1);
                self.$buf.get(masked_index)
            } else {
                None
            }
        }

        #[inline]
        fn front_mut(&mut self) -> Option<&mut T> {
            if !self.is_empty() {
                let masked_index = $mask(self, self.$writeptr - 1);
                self.$buf.get_mut(masked_index)
            } else {
                None
            }
        }
    }
}
