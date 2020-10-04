use core::ops::{Index, IndexMut};

use crate::RingBuffer;
use core::iter::FromIterator;
use core::marker::PhantomData;
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
/// # use ringbuffer::RingBuffer;
/// # use ringbuffer::GenericRingBuffer;
/// # use ringbuffer::typenum;
///
/// let mut buffer = GenericRingBuffer::<_, typenum::U2>::new();
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
pub struct GenericRingBuffer<T, Cap: ArrayLength<T>> {
    buf: GenericArray<T, Cap>,
    index: usize,
    length_counter: usize,
}

/// It is only possible to create a Generic RingBuffer if the type T in it implements Default.
/// This is because the array needs to be allocated at compile time, and needs to be filled with
/// some default value to avoid unsafe.
impl<T: Default, Cap: ArrayLength<T>> GenericRingBuffer<T, Cap> {
    /// Creates a new RingBuffer. The method is here for compatibility with the alloc version of
    /// RingBuffer. This method simply creates a default ringbuffer. The capacity is given as a
    /// type parameter.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct UninitExactIter<T, Cap> {
    count: usize,
    phantom1: PhantomData<T>,
    phantom2: PhantomData<Cap>,
}

impl<T, Cap: ArrayLength<T>> Default for UninitExactIter<T, Cap> {
    fn default() -> Self {
        Self {
            count: 0,
            phantom1: Default::default(),
            phantom2: Default::default(),
        }
    }
}

impl<T, Cap: ArrayLength<T>> Iterator for UninitExactIter<T, Cap> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.count += 1;

        if self.count <= Cap::to_usize() {
            let elem = unsafe { core::mem::MaybeUninit::<T>::uninit().assume_init() };

            Some(elem)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            Cap::to_usize() - self.count,
            Some(Cap::to_usize() - self.count),
        )
    }
}

impl<T, Cap: ArrayLength<T>> ExactSizeIterator for UninitExactIter<T, Cap> {
    fn len(&self) -> usize {
        Cap::to_usize()
    }
}

impl<T, Cap: ArrayLength<T>> GenericRingBuffer<T, Cap> {
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
        Self {
            buf: GenericArray::from_exact_iter(UninitExactIter::<T, Cap>::default()).expect(
                "UninitExactIter was made with Cap so must be the same size as the generic array.",
            ),
            index: 0,
            length_counter: 0,
        }
    }
}

impl<T: Default, Cap: ArrayLength<T>> Default for GenericRingBuffer<T, Cap> {
    /// Creates a buffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    fn default() -> Self {
        assert_ne!(Cap::to_usize(), 0);

        Self {
            buf: GenericArray::default(),
            index: 0,
            length_counter: 0,
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
        Cap::to_usize()
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

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::typenum;

    #[test]
    #[should_panic]
    fn test_no_empty() {
        let _ = GenericRingBuffer::<u32, typenum::U0>::new();
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = GenericRingBuffer::<i32, typenum::U2>::new();
        let _ = b[2];
    }

    #[test]
    fn test_uninit() {
        let mut b = unsafe { GenericRingBuffer::<_, typenum::U2>::new_uninit() };
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
    fn test_just_to_have_100_percent_coverage() {
        let mut u = UninitExactIter::<i32, typenum::U2>::default();
        assert_eq!(u.size_hint(), (2, Some(2)));
        assert_eq!(u.len(), 2);
        assert!(u.next().is_some());
        assert_eq!(u.size_hint(), (1, Some(1)));
        assert!(u.next().is_some());
        assert_eq!(u.size_hint(), (0, Some(0)));
        assert!(u.next().is_none());
    }

    #[test]
    #[should_panic]
    fn test_uninit_out_of_bounds() {
        let b = unsafe { GenericRingBuffer::<i32, typenum::U2>::new_uninit() };
        let _ = b[0];
    }
}
