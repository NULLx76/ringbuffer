use core::ops::{Index, IndexMut};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::marker::PhantomData;

// TODO: Remove Default <Issue #13>
pub trait RingBuffer<T: 'static + Default>:
    Default + Index<usize, Output = T> + IndexMut<usize>
{
    /// Returns the length of the internal buffer. This length grows up to the capacity and then
    /// stops growing.
    fn len(&self) -> usize;

    /// Returns true if the buffer is empty, some value between 0 and capacity.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Empties the buffer.
    fn clear(&mut self);

    /// Returns the capacity of the buffer.
    fn capacity(&self) -> usize;

    /// Gets a value relative to the current index
    fn get(&self, index: usize) -> Option<&T>;

    /// Gets a value relative to the current index mutably
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Gets a value relative to the start of the array (rarely useful, usually you want [`get`])
    fn get_absolute(&self, index: usize) -> Option<&T>;

    /// Gets a value mutably relative to the start of the array (rarely useful, usually you want [`get_mut`])
    fn get_absolute_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Pushes a value onto the buffer. Cycles around if capacity is reached.
    fn push(&mut self, e: T);

    /// Returns the value at the current index.
    /// This is the value that will be overwritten by the next push.
    fn peek(&self) -> Option<&T>;

    /// Creates an iterator over the buffer starting from the latest push.
    #[cfg(not(tarpaulin_include))]
    fn iter(&self) -> RingBufferIterator<T, Self> {
        RingBufferIterator {
            obj: &self,
            index: 0,
            phantom: Default::default(),
        }
    }

    ///  Creates a mutable iterator over the buffer starting from the latest push.
    #[cfg(not(tarpaulin_include))]
    fn iter_mut(&mut self) -> RingBufferMutIterator<T, Self> {
        RingBufferMutIterator {
            obj: self,
            index: 0,
            phantom: Default::default(),
        }
    }

    /// Converts the buffer to an vector.
    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter().cloned().collect()
    }
}

pub struct RingBufferIterator<'rb, T: 'static + Default, RB: RingBuffer<T>> {
    obj: &'rb RB,
    index: usize,
    phantom: PhantomData<T>,
}

impl<'rb, T: 'static + Default, RB: RingBuffer<T>> Iterator for RingBufferIterator<'rb, T, RB> {
    type Item = &'rb T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.obj.len() {
            let res = self.obj.get(self.index);
            self.index += 1;
            res
        } else {
            None
        }
    }
}

pub struct RingBufferMutIterator<'rb, T: 'static + Default, RB: RingBuffer<T>> {
    obj: &'rb mut RB,
    index: usize,
    phantom: PhantomData<T>,
}

impl<'rb, T: 'static + Default, RB: RingBuffer<T>> Iterator for RingBufferMutIterator<'rb, T, RB> {
    type Item = &'rb mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.obj.len() {
            let res: Option<&'_ mut T> = self.obj.get_mut(self.index);
            self.index += 1;

            // Safety:
            // This mem transmute is extending the lifetime of the returned value.
            // This is necessary because the rust borrow checker is too restrictive in giving out mutable references.
            // It thinks the iterator can give out a mutable reference, while it's also possible to mutably borrow
            // `obj` in the RingBufferMutIterator struct. This is however *never* possible because it's a private field
            // Unfortunately this is a limitation of the rust compiler. It's well explained here:
            // http://smallcultfollowing.com/babysteps/blog/2013/10/24/iterators-yielding-mutable-references/
            unsafe { core::mem::transmute::<Option<&'_ mut T>, Option<&'rb mut T>>(res) }
        } else {
            None
        }
    }
}

macro_rules! impl_ringbuffer {
    ($buf: ident, $index: ident) => {
        #[inline]
        fn get(&self, index: usize) -> Option<&T> {
            if self.len() > 0 {
                let index = (index + self.$index) % self.len();
                self.$buf.get(index)
            } else {
                None
            }
        }

        #[inline]
        fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            if self.len() > 0 {
                let index = (index + self.$index) % self.len();
                self.$buf.get_mut(index)
            } else {
                None
            }
        }

        #[inline]
        fn get_absolute(&self, index: usize) -> Option<&T> {
            if index < self.len() {
                self.$buf.get(index)
            } else {
                None
            }
        }

        #[inline]
        fn get_absolute_mut(&mut self, index: usize) -> Option<&mut T> {
            if index < self.len() {
                self.$buf.get_mut(index)
            } else {
                None
            }
        }

        #[inline]
        fn peek(&self) -> Option<&T> {
            if self.$index >= self.len() {
                None
            } else {
                self.$buf.get(self.$index)
            }
        }
    };
}
