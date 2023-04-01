use core::fmt::{self, Debug};
use core::mem::{self, MaybeUninit};
use core::num::NonZeroUsize;
use core::ops::{Index, IndexMut};

extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

use crate::ringbuffer_trait::*;

/// A [`RingBuffer`] efficiently supporting non-power-of-two sizes.
///
/// Most ring-buffers use power-of-two capacities because indices can be wrapped
/// to within the capacity very efficiently.  For non-power-of-two capacities, a
/// modulo operation is incredibly expensive, so instead conditional subtraction
/// is performed.  Because these subtractions only need to occur when the buffer
/// wraps around, its relative cost decreases as the buffer capacity increases.
pub struct ModFreeRingBuffer<T> {
    /// The data buffer.
    ///
    /// The length of this slice is the capacity of the ring buffer.  It stores
    /// the individual elements of the buffer, and uninitialized data for slots
    /// that do not contain elements.
    ///
    /// The number of initialized elements in the data buffer is computed as
    /// `dsti - srci`.  As such, if `dsti - srci == capacity`, then the buffer
    /// is full.
    data: Box<[MaybeUninit<T>]>,

    /// The source index.
    ///
    /// This is the index of the first initialized element in the data buffer,
    /// if the buffer is not empty.  This field is maintained within the range
    /// `0 .. capacity`; as such, it can be used to directly index into the data
    /// buffer.
    srci: usize,

    /// The destination index.
    ///
    /// This is the index in the data buffer where new elements will be added.
    /// Note that this index may already contain an initialized element.  This
    /// field is maintained within the range `srci ..= srci+capacity` (in more
    /// absolute terms, `0 .. 2*capacity`); it is at most one subtraction away
    /// from being a valid index into the data buffer.
    ///
    /// If the buffer is full, then this field is `srci + capacity`.  If this
    /// field is strictly greater than `capacity`, then some elements lie at the
    /// beginning of the data buffer, at positions less than the source index.
    dsti: usize,
}

impl<T> ModFreeRingBuffer<T> {
    /// Construct a new [`ModFreeRingBuffer`] with the given capacity.
    #[inline]
    pub fn new(capacity: NonZeroUsize) -> Self {
        // SAFETY: [`NonZeroUsize`] guarantees that the value is non-zero.
        unsafe { Self::new_unchecked(capacity.get()) }
    }

    /// Construct a new [`ModFreeRingBuffer`] with the given capacity, without
    /// checking that it is non-zero.
    #[inline]
    pub unsafe fn new_unchecked(capacity: usize) -> Self {
        // NOTE: Use Box::new_uninit() when it stabilizes.
        let mut data = Vec::with_capacity(capacity);
        // SAFETY: `MaybeUninit` is is never uninitialized.
        unsafe { data.set_len(capacity); }

        Self { data: data.into_boxed_slice(), srci: 0, dsti: 0 }
    }
}

impl<T> RingBuffer<T> for ModFreeRingBuffer<T> {
    #[inline]
    unsafe fn ptr_len(this: *const Self) -> usize {
        (*this).dsti - (*this).srci
    }

    #[inline]
    unsafe fn ptr_capacity(this: *const Self) -> usize {
        (*this).data.len()
    }
}

impl<T> RingBufferRead<T> for ModFreeRingBuffer<T> {
    #[inline]
    fn dequeue(&mut self) -> Option<T> {
        if self.srci < self.dsti {
            // SAFETY: `srci` is in range and `len` is non-zero, and `srci` will
            // be incremented so that the slot is marked dead for future reads.
            let slot = unsafe { self.data.get_unchecked_mut(self.srci) };
            let item = unsafe { slot.assume_init_read() };

            if self.srci + 1 == self.data.len() {
                // NOTE: We need to wrap `dsti` because:
                // - srci < dsti
                // - srci == capacity - 1
                // - capacity - 1 < dsti
                // - capacity <= dsti
                // So the new length (`dsti - srci`) would change incorrectly.
                self.srci = 0;
                self.dsti -= self.data.len();
            } else { self.srci += 1; }

            Some(item)
        } else { None }
    }

    fn skip(&mut self) {
        if self.srci < self.dsti {
            // SAFETY: `srci` is in range and `len` is non-zero, and `srci` will
            // be incremented so that the slot is marked dead for future reads.
            let slot = unsafe { self.data.get_unchecked_mut(self.srci) };
            unsafe { slot.assume_init_drop() };

            if self.srci + 1 == self.data.len() {
                self.srci = 0;
            } else { self.srci += 1; }
        }
    }
}

impl<T> RingBufferWrite<T> for ModFreeRingBuffer<T> {
    #[inline]
    fn push(&mut self, value: T) {
        let dsti = if self.dsti >= self.data.len() {
            self.dsti - self.data.len()
        } else { self.dsti };

        // SAFETY: `dsti` has been conditionally subtracted into range.
        let slot = unsafe { self.data.get_unchecked_mut(dsti) };
        let mut prev = mem::replace(slot, MaybeUninit::new(value));

        if self.dsti == self.srci + self.data.len() {
            // SAFETY: The buffer is full, so `prev` must be initialized.
            unsafe { prev.assume_init_drop() };

            self.dsti += 1;
            if self.dsti == 2 * self.data.len() {
                self.srci = 0;
                self.dsti = self.data.len();
            } else {
                self.srci += 1;
            }
        } else {
            // NOTE: The buffer is not full, so:
            // - dsti < srci + capacity
            // - dsti + 1 < 1 + srci + capacity
            // - dsti + 1 < 1 + capacity-1 + capacity
            // - dsti + 1 < 2*capacity
            self.dsti += 1;
        }
    }
}

unsafe impl<T> RingBufferExt<T> for ModFreeRingBuffer<T> {
    fn fill_with<F: FnMut() -> T>(&mut self, mut f: F) {
        self.clear();

        for slot in self.data.iter_mut() {
            *slot = MaybeUninit::new((f)());
        }

        self.srci = 0;
        self.dsti = self.data.len();
    }

    fn clear(&mut self) {
        while let Some(item) = self.dequeue() {
            let _ = item;
        }
    }

    fn get(&self, index: isize) -> Option<&T> {
        let len_s = self.len() as isize;
        let index = if len_s != 0 && (index <= -len_s || len_s < index) {
            // We are forced to perform an expensive modulo, so we try to hide
            // it behind a probably-unlikely branch.
            index.rem_euclid(len_s) as usize
        } else if len_s != 0 && (index < 0) {
            (index + len_s) as usize
        } else if len_s != 0 {
            index as usize
        } else { return None };

        // NOTE: We know that `index` is now within `self.len()`, so it must be
        // within `self.capacity()`.

        let index = if self.srci + index >= self.data.len() {
            self.srci + index - self.data.len()
        } else { self.srci + index };

        // SAFETY: We have confirmed that `index` is in `srci .. dsti`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { self.data[index].assume_init_ref() })
    }

    unsafe fn ptr_get_mut(this: *mut Self, index: isize) -> Option<*mut T> {
        let len_s = (*this).len() as isize;
        let index = if len_s != 0 && (index <= -len_s || len_s < index) {
            // We are forced to perform an expensive modulo, so we try to hide
            // it behind a probably-unlikely branch.
            index.rem_euclid(len_s) as usize
        } else if len_s != 0 && (index < 0) {
            (index + len_s) as usize
        } else if len_s != 0 {
            index as usize
        } else { return None };

        // NOTE: We know that `index` is now within `self.len()`, so it must be
        // within `self.capacity()`.

        let index = if (*this).srci + index >= (*this).data.len() {
            (*this).srci + index - (*this).data.len()
        } else { (*this).srci + index };

        // SAFETY: We have confirmed that `index` is in `srci .. dsti`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { (*this).data[index].assume_init_mut() })
    }

    fn get_absolute(&self, index: usize) -> Option<&T> {
        let capacity = self.data.len();
        if self.dsti > capacity {
            // The data buffer contains a single uninitialized part.
            if self.dsti - capacity <= index && index < self.srci {
                return None;
            }
        } else {
            // The data buffer contains two uninitialized parts.
            if index < self.srci || self.dsti <= index {
                return None;
            }
        }

        // SAFETY: We have confirmed that `index` is in `srci .. dsti`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { self.data[index].assume_init_ref() })
    }

    fn get_absolute_mut(&mut self, index: usize) -> Option<&mut T> {
        let capacity = self.data.len();
        if self.dsti > capacity {
            // The data buffer contains a single uninitialized part.
            if self.dsti - capacity <= index && index < self.srci {
                return None;
            }
        } else {
            // The data buffer contains two uninitialized parts.
            if index < self.srci || self.dsti <= index {
                return None;
            }
        }

        // SAFETY: We have confirmed that `index` is in `srci .. dsti`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { self.data[index].assume_init_mut() })
    }
}

impl<T> Index<isize> for ModFreeRingBuffer<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> IndexMut<isize> for ModFreeRingBuffer<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<T> Extend<T> for ModFreeRingBuffer<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T> FromIterator<T> for ModFreeRingBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let _ = iter;
        unimplemented!()
    }
}

impl<T> Drop for ModFreeRingBuffer<T> {
    fn drop(&mut self) {
        // Items need to be dropped manually because of [`MaybeUninit`].
        self.clear();
    }
}

impl<T: Clone> Clone for ModFreeRingBuffer<T> {
    fn clone(&self) -> Self {
        // SAFETY: We know that our capacity is non-zero.
        let mut clone = unsafe { Self::new_unchecked(self.data.len()) };
        clone.extend(self.iter().cloned());
        clone
    }
}

impl<T: PartialEq> PartialEq for ModFreeRingBuffer<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.capacity() != other.capacity() { return false; }
        self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for ModFreeRingBuffer<T> {}

impl<T: Debug> Debug for ModFreeRingBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Wrapper<'a, T> { this: &'a ModFreeRingBuffer<T> }
        impl<'a, T: Debug> Debug for Wrapper<'a, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.this.iter()).finish()
            }
        }

        f.debug_struct("ModFreeRingBuffer")
            .field("srci", &self.srci).field("dsti", &self.dsti)
            .field("data", &Wrapper { this: self })
            .finish()
    }
}
