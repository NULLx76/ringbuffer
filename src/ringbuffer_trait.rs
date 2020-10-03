
use core::ops::Index;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// TODO: Remove Default <Issue #13>
pub trait RingBuffer<'a, 'b, T: 'static + Default>: Default + Index<usize> {
    type Iter: Iterator<Item = &'a T>;
    type IterMut: Iterator<Item = &'b mut T>;

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

    /// Pushes a value onto the buffer. Cycles around if capacity is reached.
    fn push(&mut self, e: T);

    /// Returns the value at the current index.
    /// This is the value that will be overwritten by the next push.
    fn peek(&self) -> Option<&T>;

    /// Creates an iterator over the buffer starting from the latest push.
    fn iter(&'a self) -> Self::Iter;

    ///  Creates a mutable iterator over the buffer starting from the latest push.
    fn iter_mut(&'b mut self) -> Self::IterMut;

    /// Converts the buffer to an vector.
    #[cfg(feature = "alloc")]
    fn as_vec(&self) -> Vec<&T>;

    /// Converts the buffer to an vector.
    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<T> where T: Clone {
        self.as_vec().into_iter().cloned().collect()
    }
}
