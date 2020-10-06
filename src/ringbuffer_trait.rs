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
/// can be found in the [`WritableRingbuffer`], [`ReadableRingbuffer`] and [`RingBufferExt`] traits.
///
/// This trait is not object safe, so can't be used dynamically. However it is possible to
/// define a generic function over types implementing RingBuffer.
pub trait RingBuffer<T: 'static + Default>: Default + FromIterator<T> {
    /// Returns the length of the internal buffer.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
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
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// assert!(buffer.try_push(1).is_ok());
    /// assert!(buffer.try_push(2).is_ok());
    /// // fails because the queue is full
    /// assert_eq!(buffer.try_push(3), Err(3));
    /// ```
    fn try_push(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            Err(item)
        } else {
            self.push(item);
            Ok(())
        }
    }
}

/// Defines RingBuffer methods necessary to read from the ringbuffer. This includes dequeue.
pub trait ReadableRingbuffer<T: 'static + Default>: RingBuffer<T> {
    /// Pops an item off the queue, but does not return it. Instead it is dropped.
    /// If the ringbuffer is empty, this function is a nop.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, ReadableRingbuffer};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// buffer.skip();
    /// assert!(buffer.is_empty());
    /// ```
    fn skip(&mut self);

    /// Dequeues the an item from the ringbuffer and returns an owned version.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, ReadableRingbuffer};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// assert_eq!(buffer.pop(), Some(1));
    /// ```
    fn pop(&mut self) -> Option<T>;
}

/// Defines Ringbuffer methods necessary to mutate data inside the ringbuffer or query data in the middle
/// of the ringbuffer.
///
/// Notably, the thread safe ringbuffer does *not* implement this trait because
/// to modify or read data in the middle of the buffer would require locking, something we want to avoid.
pub trait RingBufferExt<T: 'static + Default>:
    RingBuffer<T>
    + WritableRingbuffer<T>
    + ReadableRingbuffer<T>
    + Index<usize, Output = T>
    + IndexMut<usize>
{
    /// Returns true if elem is in the ringbuffer.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// assert!(buffer.contains(&1))
    /// ```
    fn contains(&self, elem: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|i| i == elem)
    }

    /// Returns a reference to the value at the back of the queue.
    /// This is the item that will be dequeued next, and was pushed longest ago.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// buffer.push(2);
    /// assert_eq!(buffer.back(), Some(&1))
    /// ```
    #[inline]
    fn back(&self) -> Option<&T> {
        self.get(0)
    }

    /// Returns a mutable reference to the value at the back of the queue.
    /// This is the item that will be dequeued next, and was pushed longest ago.
    /// See [`back`](Self::back)
    #[inline]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Returns a reference to the value at the front of the queue.
    /// This is the item that was last pushed.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// buffer.push(2);
    /// assert_eq!(buffer.front(), Some(&2))
    /// ```
    fn front(&self) -> Option<&T>;

    /// Returns a mutable reference to the value at the front of the queue.
    /// This is the item that was last pushed.
    /// See [`front`](Self::front)
    fn front_mut(&mut self) -> Option<&mut T>;

    /// Returns a reference to a value relative to the read end of the ringbuffer.
    /// `get(0)` is the item that will be dequeued next, and is the same as [`back`](Self::back)
    /// `get(1)` is the item that will be dequeued after `get(0)`
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// buffer.push(2);
    /// assert_eq!(buffer.get(1), Some(&2))
    /// ```
    fn get(&self, index: usize) -> Option<&T>;

    /// Returns a mutable reference to a value relative to the read end of the ringbuffer.
    /// See [`get`](Self::get)
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Creates an iterator over the buffer starting from the back (the item pushed longest ago)
    /// and ending at the element most recently pushed.
    /// ```
    /// # use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
    /// let mut buffer = AllocRingBuffer::with_capacity(2);
    /// buffer.push(1);
    /// buffer.push(2);
    ///
    /// let mut it = buffer.iter();
    /// assert_eq!(Some(&1), it.next());
    /// assert_eq!(Some(&2), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    #[inline]
    fn iter(&self) -> RingBufferIterator<T, Self> {
        RingBufferIterator::new(self)
    }

    /// Creates a mutable iterator over the ringbuffer
    /// See [`iter`](Self::iter)
    #[inline]
    fn iter_mut(&mut self) -> RingBufferMutIterator<T, Self> {
        RingBufferMutIterator::new(self)
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
pub trait ReadWriteRingbuffer<T: 'static + Default>:
    RingBuffer<T> + WritableRingbuffer<T> + ReadableRingbuffer<T>
{
}

impl<S, T: 'static + Default> ReadWriteRingbuffer<T> for S where
    S: RingBuffer<T> + ReadableRingbuffer<T> + WritableRingbuffer<T>
{
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
    };
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
    };
}
