use crate::ringbuffer_trait::{RingBufferIntoIterator, RingBufferIterator, RingBufferMutIterator};
use crate::RingBuffer;
use core::iter::FromIterator;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};

/// The `ConstGenericRingBuffer` struct is a `RingBuffer` implementation which does not require `alloc` but
/// uses const generics instead.
///
/// [`ConstGenericRingBuffer`] allocates the ringbuffer on the stack, and the size must be known at
/// compile time through const-generics.
///
/// # Example
/// ```
/// use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
///
/// let mut buffer = ConstGenericRingBuffer::<_, 2>::new();
///
/// // First entry of the buffer is now 5.
/// buffer.push(5);
///
/// // The last item we pushed is 5
/// assert_eq!(buffer.back(), Some(&5));
///
/// // Second entry is now 42.
/// buffer.push(42);
///
/// assert_eq!(buffer.peek(), Some(&5));
/// assert!(buffer.is_full());
///
/// // Because capacity is reached the next push will be the first item of the buffer.
/// buffer.push(1);
/// assert_eq!(buffer.to_vec(), vec![42, 1]);
/// ```
#[derive(Debug)]
pub struct ConstGenericRingBuffer<T, const CAP: usize> {
    buf: [MaybeUninit<T>; CAP],
    readptr: usize,
    writeptr: usize,
}

impl<T, const CAP: usize> From<[T; CAP]> for ConstGenericRingBuffer<T, CAP> {
    fn from(value: [T; CAP]) -> Self {
        Self {
            // Safety:
            // T has the same layout as MaybeUninit<T>
            // [T; N] has the same layout as [MaybeUninit<T>; N]
            buf: unsafe { mem::transmute_copy(&value) },
            readptr: 0,
            writeptr: CAP,
        }
    }
}

impl<T: Clone, const CAP: usize> From<&[T; CAP]> for ConstGenericRingBuffer<T, CAP> {
    fn from(value: &[T; CAP]) -> Self {
        Self::from(value.clone())
    }
}

impl<T: Clone, const CAP: usize> From<&[T]> for ConstGenericRingBuffer<T, CAP> {
    fn from(value: &[T]) -> Self {
        value.iter().cloned().collect()
    }
}

impl<T: Clone, const CAP: usize> From<&mut [T; CAP]> for ConstGenericRingBuffer<T, CAP> {
    fn from(value: &mut [T; CAP]) -> Self {
        Self::from(value.clone())
    }
}

impl<T: Clone, const CAP: usize> From<&mut [T]> for ConstGenericRingBuffer<T, CAP> {
    fn from(value: &mut [T]) -> Self {
        value.iter().cloned().collect()
    }
}

#[cfg(feature = "alloc")]
impl<T, const CAP: usize> From<alloc::vec::Vec<T>> for ConstGenericRingBuffer<T, CAP> {
    fn from(value: alloc::vec::Vec<T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<T, const CAP: usize> From<alloc::collections::VecDeque<T>> for ConstGenericRingBuffer<T, CAP> {
    fn from(value: alloc::collections::VecDeque<T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<T, const CAP: usize> From<alloc::collections::LinkedList<T>>
    for ConstGenericRingBuffer<T, CAP>
{
    fn from(value: alloc::collections::LinkedList<T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const CAP: usize> From<alloc::string::String> for ConstGenericRingBuffer<char, CAP> {
    fn from(value: alloc::string::String) -> Self {
        value.chars().collect()
    }
}

impl<const CAP: usize> From<&str> for ConstGenericRingBuffer<char, CAP> {
    fn from(value: &str) -> Self {
        value.chars().collect()
    }
}

#[cfg(feature = "alloc")]
impl<T, const CAP: usize> From<crate::GrowableAllocRingBuffer<T>>
    for ConstGenericRingBuffer<T, CAP>
{
    fn from(mut value: crate::GrowableAllocRingBuffer<T>) -> Self {
        value.drain().collect()
    }
}

#[cfg(feature = "alloc")]
impl<T, const CAP: usize> From<crate::AllocRingBuffer<T>> for ConstGenericRingBuffer<T, CAP> {
    fn from(mut value: crate::AllocRingBuffer<T>) -> Self {
        value.drain().collect()
    }
}

impl<T, const CAP: usize> Drop for ConstGenericRingBuffer<T, CAP> {
    fn drop(&mut self) {
        self.drain().for_each(drop);
    }
}

impl<T: Clone, const CAP: usize> Clone for ConstGenericRingBuffer<T, CAP> {
    fn clone(&self) -> Self {
        let mut new = ConstGenericRingBuffer::<T, CAP>::new();
        self.iter().cloned().for_each(|i| new.push(i));
        new
    }
}

// We need to manually implement PartialEq because MaybeUninit isn't PartialEq
impl<T: PartialEq, const CAP: usize> PartialEq for ConstGenericRingBuffer<T, CAP> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            for (a, b) in self.iter().zip(other.iter()) {
                if a != b {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl<T: PartialEq, const CAP: usize> Eq for ConstGenericRingBuffer<T, CAP> {}

impl<T, const CAP: usize> ConstGenericRingBuffer<T, CAP> {
    const ERROR_CAPACITY_IS_NOT_ALLOWED_TO_BE_ZERO: () =
        assert!(CAP != 0, "Capacity is not allowed to be zero");

    /// Creates a const generic ringbuffer, size is passed as a const generic.
    ///
    /// Note that the size does not have to be a power of two, but that not using a power
    /// of two might be significantly (up to 3 times) slower.
    #[inline]
    #[must_use]
    pub const fn new<const N: usize>() -> Self
    where
        ConstGenericRingBuffer<T, CAP>: From<ConstGenericRingBuffer<T, N>>,
    {
        #[allow(clippy::let_unit_value)]
        let _ = Self::ERROR_CAPACITY_IS_NOT_ALLOWED_TO_BE_ZERO;

        // allow here since we are constructing an array of MaybeUninit<T>
        // which explicitly *is* defined behavior
        // https://rust-lang.github.io/rust-clippy/master/index.html#uninit_assumed_init
        #[allow(clippy::uninit_assumed_init)]
        Self {
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            writeptr: 0,
            readptr: 0,
        }
    }
}

/// Get a reference from the buffer without checking it is initialized
/// Caller MUST be sure this index is initialized, or undefined behavior will happen
unsafe fn get_unchecked<'a, T, const N: usize>(
    rb: *const ConstGenericRingBuffer<T, N>,
    index: usize,
) -> &'a T {
    (*rb).buf[index]
        .as_ptr()
        .as_ref()
        .expect("const array ptr shouldn't be null!")
}

/// Get a mutable reference from the buffer without checking it is initialized
/// Caller MUST be sure this index is initialized, or undefined behavior will happen
unsafe fn get_unchecked_mut<T, const N: usize>(
    rb: *mut ConstGenericRingBuffer<T, N>,
    index: usize,
) -> *mut T {
    (*rb).buf[index]
        .as_mut_ptr()
        .as_mut()
        .expect("const array ptr shouldn't be null!")
}

impl<T, const CAP: usize> IntoIterator for ConstGenericRingBuffer<T, CAP> {
    type Item = T;
    type IntoIter = RingBufferIntoIterator<T, Self>;

    fn into_iter(self) -> Self::IntoIter {
        RingBufferIntoIterator::new(self)
    }
}

impl<'a, T, const CAP: usize> IntoIterator for &'a ConstGenericRingBuffer<T, CAP> {
    type Item = &'a T;
    type IntoIter = RingBufferIterator<'a, T, ConstGenericRingBuffer<T, CAP>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const CAP: usize> IntoIterator for &'a mut ConstGenericRingBuffer<T, CAP> {
    type Item = &'a mut T;
    type IntoIter = RingBufferMutIterator<'a, T, ConstGenericRingBuffer<T, CAP>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, const CAP: usize> ConstGenericRingBuffer<T, CAP> {
    /// splits the ringbuffer into two slices. One from the old pointer to the end of the buffer,
    /// and one from the start of the buffer to the new pointer
    ///
    /// # Safety
    /// Only safe when old != new
    #[inline]
    unsafe fn split_pointer_move(
        &mut self,
        old: usize,
        new: usize,
    ) -> (&mut [MaybeUninit<T>], &mut [MaybeUninit<T>]) {
        let old_mod = crate::mask_modulo(CAP, old);
        let new_mod = crate::mask_modulo(CAP, new);

        if old_mod < new_mod {
            // if there's no wrapping, nice! we can just return one slice
            (&mut self.buf[old_mod..new_mod], &mut [])
        } else {
            // the first part is from old_mod to CAP
            let (start, p1) = self.buf.split_at_mut(old_mod);

            // and the second part from 0 to new_mod
            let (p2, _) = start.split_at_mut(new_mod);

            (p1, p2)
        }
    }

    /// # Safety
    /// Only safe when `CAP` >= `BATCH_SIZE`
    #[inline]
    unsafe fn extend_from_arr_batch<const BATCH_SIZE: usize>(&mut self, data: [T; BATCH_SIZE]) {
        debug_assert!(CAP >= BATCH_SIZE);

        // algorithm to push 1 item:
        //
        // if self.is_full() {
        //     let previous_value = mem::replace(
        //         &mut self.buf[crate::mask_modulo(CAP, self.readptr)],
        //         MaybeUninit::uninit(),
        //     );
        //     // make sure we drop whatever is being overwritten
        //     // SAFETY: the buffer is full, so this must be initialized
        //     //       : also, index has been masked
        //     // make sure we drop because it won't happen automatically
        //     unsafe {
        //         drop(previous_value.assume_init());
        //     }
        //     self.readptr += 1;
        // }
        // let index = crate::mask_modulo(CAP, self.writeptr);
        // self.buf[index] = MaybeUninit::new(value);
        // self.writeptr += 1;

        let old_len = self.len();

        let old_writeptr = self.writeptr;
        let old_readptr = self.readptr;

        // so essentially, we need to update the write pointer by Self::BATCH_SIZE
        self.writeptr += BATCH_SIZE;

        // but maybe we also need to update the readptr
        // first we calculate if we will be full. if not, no need to update the readptr
        let num_items_until_full = self.capacity() - old_len;
        if num_items_until_full < BATCH_SIZE {
            // the difference is how much the read ptr needs to move
            self.readptr += BATCH_SIZE - num_items_until_full;

            debug_assert_ne!(old_readptr, self.readptr);

            // if readptr moves, we also need to free some items.
            // Safety: same safety guarantees as this function and old != new by the assertion above
            let (p1, p2) = unsafe { self.split_pointer_move(old_readptr, self.readptr) };
            // assertion: we can never be in a situation where we have to drop more than a batch size of items
            debug_assert!(p1.len() + p2.len() <= BATCH_SIZE);

            for i in p1 {
                i.assume_init_drop();
            }
            for i in p2 {
                i.assume_init_drop();
            }
        }

        debug_assert_ne!(old_writeptr, self.writeptr);
        // now we need to write some items between old_writeptr and self.writeptr
        // Safety: same safety guarantees as this function and old != new by the assertion above
        let (p1, p2) = unsafe { self.split_pointer_move(old_writeptr, self.writeptr) };
        // assertion: we can never be in a situation where we have to write more than a batch size of items
        debug_assert!(
            p1.len() + p2.len() <= BATCH_SIZE,
            "p1: {}; p2: {}; batch: {}",
            p1.len(),
            p2.len(),
            BATCH_SIZE
        );

        // if we are lucky, we're not on the boundary so either p1 or p2 has a length of Self::BATCH_SIZE
        if p1.len() == BATCH_SIZE {
            for (index, i) in data.into_iter().enumerate() {
                p1[index] = MaybeUninit::new(i);
            }
        } else if p2.len() == BATCH_SIZE {
            for (index, i) in data.into_iter().enumerate() {
                p2[index] = MaybeUninit::new(i);
            }
        } else {
            // oof, unfortunately we're on a boundary

            // iterate over the data
            let mut data_iter = data.into_iter();

            // put p1.len() in p1
            for i in p1 {
                let next_item = data_iter.next();
                // Safety: p1.len() + p2.len() <= Self::BATCH_SIZE so the two loops here
                // together cannot run for more than Self::BATCH_SIZE iterations
                *i = MaybeUninit::new(unsafe { next_item.unwrap_unchecked() });
            }

            // put p2.len() in p2
            for i in p2 {
                let next_item = data_iter.next();
                // Safety: p1.len() + p2.len() <= Self::BATCH_SIZE so the two loops here
                // together cannot run for more than Self::BATCH_SIZE iterations
                *i = MaybeUninit::new(unsafe { next_item.unwrap_unchecked() });
            }
        }
    }

    #[inline]
    fn fill_batch<const BATCH_SIZE: usize>(
        batch: &mut [MaybeUninit<T>; BATCH_SIZE],
        iter: &mut impl Iterator<Item = T>,
    ) -> usize {
        for (index, b) in batch.iter_mut().enumerate() {
            if let Some(i) = iter.next() {
                *b = MaybeUninit::new(i);
            } else {
                return index;
            }
        }

        BATCH_SIZE
    }

    #[inline]
    fn extend_batched<const BATCH_SIZE: usize>(&mut self, mut other: impl Iterator<Item = T>) {
        // SAFETY: if CAP < Self::BATCH_SIZE we can't run extend_from_arr_batch so we catch that here
        if CAP < BATCH_SIZE {
            for i in other {
                self.push(i);
            }
        } else {
            // Safety: assume init to MaybeUninit slice is safe
            let mut batch: [MaybeUninit<T>; BATCH_SIZE] =
                unsafe { MaybeUninit::uninit().assume_init() };

            // repeat until we find an empty batch
            loop {
                // fill up a batch
                let how_full = Self::fill_batch(&mut batch, &mut other);

                // if the batch isn't complete, individually add the items from that batch
                if how_full < BATCH_SIZE {
                    for b in batch.iter().take(how_full) {
                        // Safety: fill_batch filled up at least `how_full` items so if we iterate
                        // until there this is safe
                        self.push(unsafe { b.assume_init_read() });
                    }

                    // then we're done!
                    return;
                }

                // else the batch is full, and we can transmute it to an init slice
                let batch = unsafe {
                    mem::transmute_copy::<[MaybeUninit<T>; BATCH_SIZE], [T; BATCH_SIZE]>(&batch)
                };

                // SAFETY: if CAP < Self::BATCH_SIZE we woudn't be here
                unsafe { self.extend_from_arr_batch(batch) }
            }
        }
    }

    /// # Safety
    /// ONLY USE WHEN WORKING ON A CLEARED RINGBUFFER
    unsafe fn finish_iter<const BATCH_SIZE: usize>(&mut self, mut iter: impl Iterator<Item = T>) {
        let mut index = 0;
        for i in iter.by_ref() {
            self.buf[index] = MaybeUninit::new(i);
            index += 1;

            if index > CAP - 1 {
                break;
            }
        }

        if index < CAP {
            // we set writepointer to however many elements we managed to write (up to CAP-1)
            // WARNING: ONLY WORKS WHEN WORKING ON A CLEARED RINGBUFFER
            self.writeptr = index;
        } else {
            self.writeptr = CAP;
            self.extend_batched::<BATCH_SIZE>(iter);
        }
    }
}

impl<T, const CAP: usize> Extend<T> for ConstGenericRingBuffer<T, CAP> {
    /// NOTE: correctness (but not soundness) of extend depends on `size_hint` on iter being correct.
    #[inline]
    fn extend<A: IntoIterator<Item = T>>(&mut self, iter: A) {
        const BATCH_SIZE: usize = 128;

        let iter = iter.into_iter();

        let (lower, _) = iter.size_hint();

        if lower >= CAP {
            // if there are more elements in our iterator than we have size in the ringbuffer
            // drain the ringbuffer
            self.clear();

            // we need exactly CAP elements.
            // so we need to drop until the number of elements in the iterator is exactly CAP
            let num_we_can_drop = lower - CAP;

            let iter = iter.skip(num_we_can_drop);

            // Safety: clear above
            unsafe { self.finish_iter::<BATCH_SIZE>(iter) };
        } else if self.is_empty() {
            self.clear();

            // Safety: clear above
            unsafe { self.finish_iter::<BATCH_SIZE>(iter) };
        } else {
            self.extend_batched::<BATCH_SIZE>(iter);
        }
    }
}

unsafe impl<T, const CAP: usize> RingBuffer<T> for ConstGenericRingBuffer<T, CAP> {
    #[inline]
    unsafe fn ptr_capacity(_: *const Self) -> usize {
        CAP
    }

    #[inline]
    unsafe fn ptr_buffer_size(_: *const Self) -> usize {
        CAP
    }

    impl_ringbuffer!(readptr, writeptr);

    #[inline]
    fn push(&mut self, value: T) {
        if self.is_full() {
            let previous_value = mem::replace(
                &mut self.buf[crate::mask_modulo(CAP, self.readptr)],
                MaybeUninit::uninit(),
            );
            // make sure we drop whatever is being overwritten
            // SAFETY: the buffer is full, so this must be initialized
            //       : also, index has been masked
            // make sure we drop because it won't happen automatically
            unsafe {
                drop(previous_value.assume_init());
            }
            self.readptr += 1;
        }
        let index = crate::mask_modulo(CAP, self.writeptr);
        self.buf[index] = MaybeUninit::new(value);
        self.writeptr += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let index = crate::mask_modulo(CAP, self.readptr);
            let res = mem::replace(&mut self.buf[index], MaybeUninit::uninit());
            self.readptr += 1;

            // Safety: the fact that we got this maybeuninit from the buffer (with mask) means that
            // it's initialized. If it wasn't the is_empty call would have caught it. Values
            // are always initialized when inserted so this is safe.
            unsafe { Some(res.assume_init()) }
        }
    }

    impl_ringbuffer_ext!(
        get_unchecked,
        get_unchecked_mut,
        readptr,
        writeptr,
        crate::mask_modulo
    );

    #[inline]
    fn fill_with<F: FnMut() -> T>(&mut self, mut f: F) {
        self.clear();
        self.readptr = 0;
        self.writeptr = CAP;
        self.buf.fill_with(|| MaybeUninit::new(f()));
    }
}

impl<T, const CAP: usize> Default for ConstGenericRingBuffer<T, CAP> {
    /// Creates a buffer with a capacity specified through the Cap type parameter.
    /// # Panics
    /// Panics if `CAP` is 0
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<RB, const CAP: usize> FromIterator<RB> for ConstGenericRingBuffer<RB, CAP> {
    fn from_iter<T: IntoIterator<Item = RB>>(iter: T) -> Self {
        let mut res = Self::default();
        for i in iter {
            res.push(i);
        }

        res
    }
}

impl<T, const CAP: usize> Index<usize> for ConstGenericRingBuffer<T, CAP> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T, const CAP: usize> IndexMut<usize> for ConstGenericRingBuffer<T, CAP> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::hint::black_box;
    use core::ops::Range;

    #[test]
    fn test_not_power_of_two() {
        let mut rb = ConstGenericRingBuffer::<usize, 10>::new();
        const NUM_VALS: usize = 1000;

        // recycle the ringbuffer a bunch of time to see if noneof the logic
        // messes up
        for _ in 0..100 {
            for i in 0..NUM_VALS {
                rb.enqueue(i);
            }
            assert!(rb.is_full());

            for i in 0..10 {
                assert_eq!(Some(i + NUM_VALS - rb.capacity()), rb.dequeue())
            }

            assert!(rb.is_empty())
        }
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = ConstGenericRingBuffer::<i32, 2>::new();
        let _ = b[2];
    }

    #[test]
    fn test_extend() {
        let mut buf = ConstGenericRingBuffer::<u8, 4>::new();
        (0..4).for_each(|_| buf.push(0));

        let new_data = [0, 1, 2];
        buf.extend(new_data);

        let expected = [0, 0, 1, 2];

        for i in 0..4 {
            let actual = buf[i];
            let expected = expected[i];
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_extend_with_overflow() {
        let mut buf = ConstGenericRingBuffer::<u8, 8>::new();
        (0..8).for_each(|_| buf.push(0));

        let new_data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        buf.extend(new_data);

        let expected = [2, 3, 4, 5, 6, 7, 8, 9];

        for i in 0..8 {
            let actual = buf[i];
            let expected = expected[i];
            assert_eq!(actual, expected);
        }
    }

    struct Weirderator<T: IntoIterator>(<T as IntoIterator>::IntoIter, SizeHint);

    impl<T: IntoIterator> Iterator for Weirderator<T> {
        type Item = <T as IntoIterator>::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let (lower, upper) = self.0.size_hint();

            match self.1 {
                SizeHint::TooHigh => (lower + 10, upper),
                SizeHint::TooLow => (lower - 10, upper),
                SizeHint::Good => (lower, upper),
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum SizeHint {
        TooHigh,
        TooLow,
        Good,
    }

    struct IntoWeirderator<T: IntoIterator>(pub T, SizeHint);

    impl<T: IntoIterator> IntoIterator for IntoWeirderator<T>
    where
        <T as IntoIterator>::IntoIter: Sized,
    {
        type Item = <T as IntoIterator>::Item;
        type IntoIter = Weirderator<T>;

        fn into_iter(self) -> Self::IntoIter {
            Weirderator(self.0.into_iter(), self.1)
        }
    }

    #[test]
    // tests whether we correctly drop items when the batch crosses the boundary
    fn boundary_drop_extend() {
        for n in 50..300 {
            let mut a = ConstGenericRingBuffer::<_, 128>::new();

            for i in 0..n {
                a.push(i);
            }

            a.extend(0..n);

            for _ in 0..128 {
                let _ = black_box(a.dequeue());
            }
        }
    }

    #[test]
    fn test_verify_extend() {
        extern crate std;

        macro_rules! for_cap {
            ($cap: expr) => {{
                const CAP: usize = $cap;

                for start in 0..5 {
                    for size in [SizeHint::TooLow, SizeHint::Good, SizeHint::TooHigh] {
                        std::println!("{start} {size:?}");

                        let mut rb = ConstGenericRingBuffer::<usize, CAP>::new();
                        for i in 0..start {
                            rb.push(i);
                        }

                        rb.extend(Weirderator::<Range<usize>>(0..CAP, size));
                        rb.push(17);
                        rb.push(18);
                        rb.push(19);

                        for _ in 0..CAP {
                            let _ = rb.dequeue();
                        }

                        let mut rb = ConstGenericRingBuffer::<usize, CAP>::new();
                        for i in 0..start {
                            rb.push(i);
                        }

                        rb.extend(Weirderator::<Range<usize>>(0..(CAP + 1), size));
                        rb.push(18);
                        rb.push(19);

                        for _ in 0..CAP {
                            let _ = rb.dequeue();
                        }

                        let mut rb = ConstGenericRingBuffer::<usize, CAP>::new();
                        for i in 0..start {
                            rb.push(i);
                        }

                        rb.extend(Weirderator::<Range<usize>>(0..(CAP + 2), size));
                        rb.push(19);

                        for _ in 0..CAP {
                            let _ = rb.dequeue();
                        }
                    }
                }
            };};
        }

        for_cap!(17);
        for_cap!(70);
        for_cap!(128);
    }

    #[cfg(test)]
    mod tests {
        use crate::{AllocRingBuffer, ConstGenericRingBuffer, GrowableAllocRingBuffer, RingBuffer};
        use alloc::collections::{LinkedList, VecDeque};
        use alloc::string::ToString;
        use alloc::vec;

        #[test]
        fn from() {
            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from([1, 2, 3]).to_vec(),
                vec![1, 2, 3]
            );

            let v: &[i32; 3] = &[1, 2, 3];
            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from(v).to_vec(),
                vec![1, 2, 3]
            );

            let v: &[i32] = &[1, 2, 3];
            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from(v).to_vec(),
                vec![1, 2, 3]
            );

            let v: &mut [i32; 3] = &mut [1, 2, 3];
            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from(v).to_vec(),
                vec![1, 2, 3]
            );

            let v: &mut [i32] = &mut [1, 2, 3];
            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from(v).to_vec(),
                vec![1, 2, 3]
            );

            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from(vec![1, 2, 3]).to_vec(),
                vec![1, 2, 3]
            );
            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from(
                    vec![1, 2, 3].into_iter().collect::<VecDeque<_>>()
                )
                .to_vec(),
                vec![1, 2, 3]
            );
            assert_eq!(
                ConstGenericRingBuffer::<i32, 3>::from(
                    vec![1, 2, 3].into_iter().collect::<LinkedList<_>>()
                )
                .to_vec(),
                vec![1, 2, 3]
            );
            assert_eq!(
                ConstGenericRingBuffer::<_, 3>::from("abc".to_string()).to_vec(),
                vec!['a', 'b', 'c']
            );
            assert_eq!(
                ConstGenericRingBuffer::<_, 3>::from("abc").to_vec(),
                vec!['a', 'b', 'c']
            );
            assert_eq!(
                ConstGenericRingBuffer::<_, 3>::from(GrowableAllocRingBuffer::from(vec![1, 2, 3]))
                    .to_vec(),
                vec![1, 2, 3]
            );
            assert_eq!(
                ConstGenericRingBuffer::<_, 3>::from(AllocRingBuffer::from(vec![1, 2, 3])).to_vec(),
                vec![1, 2, 3]
            );
        }
    }
}
