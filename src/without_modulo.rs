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
    buf: Box<[MaybeUninit<T>]>,

    /// The source index.
    ///
    /// This is the index of the first initialized element in the data buffer,
    /// if the buffer is not empty.  This field is maintained within the range
    /// `0 .. capacity`; as such, it can be used to directly index into the data
    /// buffer.
    src: usize,

    /// The length of the buffer.
    ///
    /// This is a count of the number of initialized elements in the buffer.  If
    /// it is equal to the capacity, then the buffer is full.  This field is in
    /// the range `0 ..= capacity`; once added to the source index, it may need
    /// to be conditionally subtracted by `capacity` to point to a valid index
    /// in the data buffer.
    len: usize,
}

impl<T> ModFreeRingBuffer<T> {
    /// Construct a new [`ModFreeRingBuffer`] with the given capacity.
    pub fn new(capacity: NonZeroUsize) -> Self {
        // SAFETY: [`NonZeroUsize`] guarantees that the value is non-zero.
        unsafe { Self::new_unchecked(capacity.get()) }
    }

    /// Construct a new [`ModFreeRingBuffer`] with the given capacity, without
    /// checking that it is non-zero.
    pub unsafe fn new_unchecked(capacity: usize) -> Self {
        // NOTE: Use Box::new_uninit() when it stabilizes.
        let mut buf = Vec::with_capacity(capacity);
        // SAFETY: `MaybeUninit` is is never uninitialized.
        unsafe { buf.set_len(capacity); }

        Self { buf: buf.into_boxed_slice(), src: 0, len: 0 }
    }
}

impl<T> RingBuffer<T> for ModFreeRingBuffer<T> {
    #[inline]
    unsafe fn ptr_len(this: *const Self) -> usize {
        (*this).len
    }

    #[inline]
    unsafe fn ptr_capacity(this: *const Self) -> usize {
        (*this).buf.len()
    }
}

impl<T> RingBufferRead<T> for ModFreeRingBuffer<T> {
    #[inline]
    fn dequeue(&mut self) -> Option<T> {
        if self.len != 0 {
            // SAFETY: `src` is in range and `len` is non-zero, and `src` will
            // be incremented so that the slot is marked dead for future reads.
            let slot = unsafe { self.buf.get_unchecked_mut(self.src) };
            let item = unsafe { slot.assume_init_read() };

            self.len -= 1;
            if self.src + 1 == self.buf.len() {
                self.src = 0;
            } else { self.src += 1; }

            Some(item)
        } else { None }
    }

    fn skip(&mut self) {
        if self.len != 0 {
            // SAFETY: `src` is in range and `len` is non-zero, and `src` will
            // be incremented so that the slot is marked dead for future reads.
            let slot = unsafe { self.buf.get_unchecked_mut(self.src) };
            unsafe { slot.assume_init_drop() };

            self.len -= 1;
            if self.src + 1 == self.buf.len() {
                self.src = 0;
            } else { self.src += 1; }
        }
    }
}

impl<T> RingBufferWrite<T> for ModFreeRingBuffer<T> {
    #[inline]
    fn push(&mut self, value: T) {
        let dst = if self.src + self.len >= self.buf.len() {
            self.src + self.len - self.buf.len()
        } else { self.src + self.len };

        // SAFETY: `dst` has been conditionally subtracted into range.
        let slot = unsafe { self.buf.get_unchecked_mut(dst) };
        let mut prev = mem::replace(slot, MaybeUninit::new(value));

        if self.len == self.buf.len() {
            // SAFETY: The buffer is full, so `prev` must be initialized.
            unsafe { prev.assume_init_drop() };

            if self.src + 1 == self.buf.len() {
                self.src = 0;
            } else {
                self.src += 1;
            }
        } else {
            self.len += 1;
        }
    }
}

unsafe impl<T> RingBufferExt<T> for ModFreeRingBuffer<T> {
    fn fill_with<F: FnMut() -> T>(&mut self, mut f: F) {
        self.clear();

        for slot in self.buf.iter_mut() {
            *slot = MaybeUninit::new((f)());
        }

        self.src = 0;
        self.len = self.buf.len();
    }

    fn clear(&mut self) {
        while let Some(item) = self.dequeue() {
            let _ = item;
        }
    }

    fn get(&self, index: isize) -> Option<&T> {
        let len_s = self.len as isize;
        let index = if len_s != 0 && (index <= -len_s || len_s < index) {
            // We are forced to perform an expensive modulo, so we try to hide
            // it behind a probably-unlikely branch.
            index.rem_euclid(len_s) as usize
        } else if len_s != 0 && (index < 0) {
            (index + len_s) as usize
        } else if len_s != 0 {
            index as usize
        } else { return None };

        // NOTE: We know that `index` is now within `self.len`, so it must be
        // within `self.capacity()`.

        let index = if self.src + index >= self.buf.len() {
            self.src + index - self.buf.len()
        } else { self.src + index };

        // SAFETY: We have confirmed that `index` is in `src .. dst`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { self.buf[index].assume_init_ref() })
    }

    unsafe fn ptr_get_mut(this: *mut Self, index: isize) -> Option<*mut T> {
        let len_s = (*this).len as isize;
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

        let index = if (*this).src + index >= (*this).buf.len() {
            (*this).src + index - (*this).buf.len()
        } else { (*this).src + index };

        // SAFETY: We have confirmed that `index` is in `src .. dst`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { (*this).buf[index].assume_init_mut() })
    }

    fn get_absolute(&self, index: usize) -> Option<&T> {
        let (src, dst, cap) = (self.src, self.src + self.len, self.buf.len());
        if dst > cap {
            // The data buffer contains a single uninitialized part.
            if dst - cap <= index && index < src { return None; }
        } else {
            // The data buffer contains two uninitialized parts.
            if index < src || dst <= index { return None; }
        }

        // SAFETY: We have confirmed that `index` is in `src .. dst`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { self.buf[index].assume_init_ref() })
    }

    fn get_absolute_mut(&mut self, index: usize) -> Option<&mut T> {
        let (src, dst, cap) = (self.src, self.src + self.len, self.buf.len());
        if dst > cap {
            // The data buffer contains a single uninitialized part.
            if dst - cap <= index && index < src { return None; }
        } else {
            // The data buffer contains two uninitialized parts.
            if index < src || dst <= index { return None; }
        }

        // SAFETY: We have confirmed that `index` is in `src .. dst`, so we
        // are definitely referring to an element that is initialized.
        Some(unsafe { self.buf[index].assume_init_mut() })
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
        let mut clone = unsafe { Self::new_unchecked(self.buf.len()) };
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
            .field("src", &self.src).field("len", &self.len)
            .field("data", &Wrapper { this: self })
            .finish()
    }
}
