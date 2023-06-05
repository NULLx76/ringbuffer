use crate::with_alloc::alloc_ringbuffer::RingbufferMode;
use crate::{AllocRingBuffer, RingBuffer, RingBufferExt, RingBufferRead, RingBufferWrite};
use alloc::collections::VecDeque;
use core::ops::{Deref, DerefMut, Index, IndexMut};

/// A growable ringbuffer. Once capacity is reached, the size is doubled.
/// Wrapper of the built-in [`VecDeque`](std::collections::VecDeque) struct
///
/// The reason this is a wrapper, is that we want RingBuffers to implement `Index<isize>`,
/// which we cannot do for remote types like `VecDeque`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrowableAllocRingBuffer<T>(VecDeque<T>);

impl<T, const N: usize> From<[T; N]> for GrowableAllocRingBuffer<T> {
    fn from(value: [T; N]) -> Self {
        Self(VecDeque::from(value))
    }
}

impl<T> From<VecDeque<T>> for GrowableAllocRingBuffer<T> {
    fn from(value: VecDeque<T>) -> Self {
        Self(value)
    }
}

impl<T, MODE: RingbufferMode> From<AllocRingBuffer<T, MODE>> for GrowableAllocRingBuffer<T> {
    fn from(mut v: AllocRingBuffer<T, MODE>) -> GrowableAllocRingBuffer<T> {
        let mut rb = GrowableAllocRingBuffer::new();
        rb.extend(v.drain());
        rb
    }
}

impl<T> Deref for GrowableAllocRingBuffer<T> {
    type Target = VecDeque<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for GrowableAllocRingBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for GrowableAllocRingBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AsRef<VecDeque<T>> for GrowableAllocRingBuffer<T> {
    fn as_ref(&self) -> &VecDeque<T> {
        &self.0
    }
}

impl<T> GrowableAllocRingBuffer<T> {
    /// Creates an empty ringbuffer.
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    /// Creates an empty ringbuffer with space for at least capacity elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }
}

impl<T> RingBuffer<T> for GrowableAllocRingBuffer<T> {
    unsafe fn ptr_len(rb: *const Self) -> usize {
        (*rb).0.len()
    }

    unsafe fn ptr_capacity(rb: *const Self) -> usize {
        (*rb).0.capacity()
    }
}

impl<T> RingBufferRead<T> for GrowableAllocRingBuffer<T> {
    fn dequeue(&mut self) -> Option<T> {
        self.pop_front()
    }

    impl_ringbuffer_read!();
}

impl<T> RingBufferWrite<T> for GrowableAllocRingBuffer<T> {
    fn push(&mut self, value: T) {
        self.push_back(value)
    }
}

impl<T> Extend<T> for GrowableAllocRingBuffer<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<T> Index<isize> for GrowableAllocRingBuffer<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> IndexMut<isize> for GrowableAllocRingBuffer<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<T> FromIterator<T> for GrowableAllocRingBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(VecDeque::from_iter(iter))
    }
}

unsafe impl<T> RingBufferExt<T> for GrowableAllocRingBuffer<T> {
    fn fill_with<F: FnMut() -> T>(&mut self, mut f: F) {
        self.clear();
        let initial_capacity = self.0.capacity();
        for _ in 0..initial_capacity {
            self.0.push_back(f())
        }

        debug_assert_eq!(initial_capacity, self.0.capacity())
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn get(&self, index: isize) -> Option<&T> {
        if self.is_empty() {
            None
        } else if index >= 0 {
            self.0.get(crate::mask_modulo(self.0.len(), index as usize))
        } else {
            let positive_index = -index as usize - 1;
            let masked = crate::mask_modulo(self.0.len(), positive_index);
            let index = self.0.len() - 1 - masked;

            self.0.get(index)
        }
    }

    unsafe fn ptr_get_mut(rb: *mut Self, index: isize) -> Option<*mut T> {
        #[allow(trivial_casts)]
        if RingBuffer::ptr_len(rb) == 0 {
            None
        } else if index >= 0 {
            (*rb).0.get_mut(index as usize)
        } else {
            let len = Self::ptr_len(rb);

            let positive_index = -index as usize + 1;
            let masked = crate::mask_modulo(len, positive_index);
            let index = len - 1 - masked;

            (*rb).0.get_mut(index)
        }
        .map(|i| i as *mut T)
    }

    fn get_absolute(&self, _index: usize) -> Option<&T> {
        unimplemented!()
    }

    fn get_absolute_mut(&mut self, _index: usize) -> Option<&mut T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AllocRingBuffer, GrowableAllocRingBuffer, RingBuffer, RingBufferRead, RingBufferWrite,
    };

    #[test]
    fn test_convert() {
        let mut a = GrowableAllocRingBuffer::new();
        a.push(0);
        a.push(1);

        let mut b: AllocRingBuffer<_, _> = a.into();
        assert_eq!(b.capacity(), 2);
        assert_eq!(b.len(), 2);
        assert_eq!(b.dequeue(), Some(0));
        assert_eq!(b.dequeue(), Some(1));
    }

    #[test]
    fn test_convert_back() {
        let mut a = AllocRingBuffer::with_capacity(2);
        a.push(0);
        a.push(1);

        let mut b: GrowableAllocRingBuffer<_> = a.into();
        assert_eq!(b.len(), 2);
        assert!(b.capacity() >= 2);
        assert_eq!(b.dequeue(), Some(0));
        assert_eq!(b.dequeue(), Some(1));
    }
}
