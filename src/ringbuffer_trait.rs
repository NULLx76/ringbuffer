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
    Default + Index<isize, Output = T> + IndexMut<isize> + FromIterator<T>
{
    /// Returns the length of the internal buffer.
    /// This length grows up to the capacity and then stops growing.
    /// This is because when the length is reached, new items are appended at the start.
    fn len(&self) -> usize;

    // TODO: issue #21: pop feature
    /// Returns true if the buffer is entirely empty.
    /// This is currently only true when nothing has ever been pushed, or when the [`Self::clear`]
    /// function is called. This might change when the `pop` function is added with issue #21
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true when the length of the ringbuffer equals the capacity. This happens whenever
    /// more elements than capacity have been pushed to the buffer.
    #[inline]
    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Empties the buffer entirely. Sets the length to 0 but keeps the capacity allocated.
    fn clear(&mut self);

    /// Returns the capacity of the buffer.
    fn capacity(&self) -> usize;
}

/// Defines RingBuffer methods necessary to write to the ringbuffer in a
pub trait WritableRingbuffer<T: 'static + Default>: RingBuffer<T> {
    /// Pushes a value onto the buffer. Cycles around if capacity is reached.
    fn push(&mut self, value: T);

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
}

/// Defines Ringbuffer methods necessary to mutate data inside the ringbuffer or query data in the middle
/// of the ringbuffer.
///
/// Notably, the thread safe ringbuffer does *not* implement this trait because
/// to modify or read data in the middle of the buffer would require locking, something we want to avoid.
pub trait RingBufferExt<T: 'static + Default>: RingBuffer<T> + WritableRingbuffer<T> + ReadableRingbuffer<T> {
    /// Returns the value at the current index.
    /// This is the value that will be overwritten by the next push and also the value pushed
    /// the longest ago. (alias of [`Self::front`])
    #[inline]
    fn peek(&self) -> Option<&T> {
        self.front()
    }

    /// Returns true if elem is in the ringbuffer.
    fn contains(&self, elem: &T) -> bool
        where
            T: PartialEq,
    {
        self.iter().any(|i| i == elem)
    }

    /// Returns the value at the back of the queue.
    /// This is the item that was pushed most recently.
    #[inline]
    fn back(&self) -> Option<&T> {
        self.get(-1)
    }

    /// Converts the buffer to a vector. This Copies all elements in the ringbuffer.
    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<T>
        where
            T: Clone,
    {
        self.iter().cloned().collect()
    }

    /// Returns the value at the back of the queue.
    /// This is the value that will be overwritten by the next push and also the value pushed
    /// the longest ago.
    /// (alias of peek)
    #[inline]
    fn front(&self) -> Option<&T> {
        self.get(0)
    }

    /// Creates an iterator over the buffer starting from the latest push.
    /// Creates an iterator over the buffer starting from the item pushed the longest ago,
    /// and ending at the element most recently pushed.
    #[inline]
    fn iter(&self) -> RingBufferIterator<T, Self> {
        RingBufferIterator::new(self)
    }

    /// Gets a value relative to the current index. 0 is the next index to be written to with push.
    /// -1 and down are the last elements pushed and 0 and up are the items that were pushed the longest ago.
    fn get(&self, index: isize) -> Option<&T>;

    /// Gets a value relative to the start of the array (rarely useful, usually you want [`Self::get`])
    fn get_absolute(&self, index: usize) -> Option<&T>;

    /// Dequeues the top item off the ringbuffer. Returns a reference to the item. This means
    /// that lifetimes will be problematic because as long as this reference exists,
    /// you can not push to the queue. To solve this, use the pop method. This requires
    /// the item to be clone. Easily moving out of the ringbuffer is sadly impossible.
    ///
    /// Returns None when the ringbuffer is empty.
    fn dequeue_ref(&mut self) -> Option<&T>;

    /// Dequeues the top item off the ringbuffer and returns an owned version. See the [`pop_ref`](Self::pop_ref) docs
    fn dequeue(&mut self) -> Option<T> where T: Clone {
        self.dequeue_ref().cloned()
    }

    /// Returns a mutable reference to the value at the back of the queue.
    /// This is the item that was pushed most recently.
    #[inline]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(-1)
    }

    /// Returns a mutable reference to the value at the back of the queue.
    /// This is the value that will be overwritten by the next push.
    /// (alias of peek)
    #[inline]
    fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    ///  Creates a mutable iterator over the buffer starting from the latest push.
    /// Creates a mutable iterator over the buffer starting from the item pushed the longest ago,
    /// and ending at the element most recently pushed.
    #[inline]
    fn iter_mut(&mut self) -> RingBufferMutIterator<T, Self> {
        RingBufferMutIterator::new(self)
    }

    /// Gets a value mutably relative to the start of the array (rarely useful, usually you want [`Self::get_mut`])
    fn get_absolute_mut(&mut self, index: usize) -> Option<&mut T>;


    /// Gets a value relative to the current index mutably. 0 is the next index to be written to with push.
    /// -1 and down are the last elements pushed and 0 and up are the items that were pushed the longest ago.
    fn get_mut(&mut self, index: isize) -> Option<&mut T>;
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
            if self.index < self.obj.len() {
                let res = self.obj.get(self.index as isize);
                self.index += 1;
                res
            } else {
                None
            }
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
            if self.index < self.obj.len() {
                let res = self.obj.get_mut(self.index as isize);
                self.index += 1;

                res
            } else {
                None
            }
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
        fn get(&self, index: isize) -> Option<&T> {
            if !self.is_empty() {
                let index = (self.$readptr as isize + index) as usize % self.len();

                self.$buf.get(index)
            } else {
                None
            }
        }

        #[inline]
        fn get_mut(&mut self, index: isize) -> Option<&mut T> {
            if !self.is_empty() {
                let index = (self.$readptr as isize + index) as usize % self.len();

                self.$buf.get_mut(index)
            } else {
                None
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
    }
}