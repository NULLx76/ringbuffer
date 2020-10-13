#![no_std]
#![cfg_attr(feature = "const_generics", feature(const_generics))]
#![cfg_attr(feature = "const_generics", allow(incomplete_features))]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(unused_import_braces)]
#![deny(unused_results)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_qualifications)]
//! # Ringbuffer
//! [![Drone (self-hosted)](https://img.shields.io/drone/build/0x76/ringbuffer?logo=drone&server=https%3A%2F%2Fdrone.xirion.net&style=for-the-badge)](https://drone.xirion.net/0x76/ringbuffer)
//! [![Codecov](https://img.shields.io/codecov/c/github/NULLx76/ringbuffer?logo=codecov&style=for-the-badge)](https://codecov.io/gh/NULLx76/ringbuffer)
//! [![Docs.rs](https://img.shields.io/badge/docs.rs-ringbuffer-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/ringbuffer)
//! [![Crates.io](https://img.shields.io/crates/v/ringbuffer?logo=rust&style=for-the-badge)](https://crates.io/crates/ringbuffer)
//!
//! The ringbuffer crate provides safe fixed size circular buffers (ringbuffers) in rust.
//!
//! Implementations for three kinds of ringbuffers, with a mostly similar API are provided:
//!
//! | type | description |
//! | --- | --- |
//! | [`AllocRingBuffer`] | Ringbuffer allocated on the heap at runtime. This ringbuffer is still fixed size and requires alloc. |
//! | [`GenericRingBuffer`] | Ringbuffer allocated on the stack. This is done using the [`typenum`] crate to provide a const-generic like interface without needing nightly |
//! | [`ConstGenericRingBuffer`] | Ringbuffer which uses const generics to allocate on the stack. This type is feature-gated behind `const_generics` and only works in nightly rust.|
//!
//! All of these ringbuffers also implement the [`RingBuffer`] trait for their shared API surface.
//!
//! # Usage
//!
//! ```
//! use ringbuffer::{AllocRingBuffer, RingBuffer, WritableRingbuffer, RingBufferExt};
//!
//! let mut buffer = AllocRingBuffer::with_capacity(2);
//!
//! // First entry of the buffer is now 5.
//! buffer.push(5).unwrap();
//!
//! // The last item we pushed is 5
//! assert_eq!(buffer.front(), Some(&5));
//!
//! // Second entry is now 42.
//! buffer.push(42).unwrap();
//!
//! assert_eq!(buffer.back(), Some(&5));
//! assert!(buffer.is_full());
//!
//! // Because capacity is reached the next push will be the first item of the buffer.
//! buffer.push_force(1);
//! assert_eq!(buffer.to_vec(), vec![42, 1]);
//!
//! ```
//!
//! # Features
//!
//! | name | default | description |
//! | --- | --- | --- |
//! | alloc | ✓ | Disable this feature to remove the dependency on alloc. Useful for kernels. |
//! | const_generics | ✗ | Enables the ConstGenericRingBuffer. This requires nightly. |
//! | generic_array | ✓ | Disable this feature to remove the [`generic_array`] and [`typenum`] dependencies (and disables [`GenericRingBuffer`]). |
//!
//! # License
//!
//! Licensed under GNU Lesser General Public License v3.0

#[macro_use]
pub(crate) mod ringbuffer_trait;
pub use ringbuffer_trait::{
    ReadWriteRingbuffer, ReadableRingbuffer, RingBuffer, RingBufferExt, WritableRingbuffer,
};

#[cfg(feature = "alloc")]
mod with_alloc;
#[cfg(feature = "alloc")]
pub use with_alloc::AllocRingBuffer;
#[cfg(feature = "alloc")]
pub use with_alloc::RINGBUFFER_DEFAULT_CAPACITY;

// #[cfg(all(feature = "alloc", feature = "threads"))]
// mod with_threads_alloc;
// #[cfg(all(feature = "alloc", feature = "threads"))]
// pub use with_threads_alloc::ThreadAllocRingBuffer;

#[cfg(feature = "const_generics")]
mod with_const_generics;
#[cfg(feature = "const_generics")]
pub use with_const_generics::ConstGenericRingBuffer;

#[cfg(feature = "generic-array")]
mod with_generic_array;

#[cfg(feature = "generic-array")]
pub use generic_array::{typenum, ArrayLength};
#[cfg(feature = "generic-array")]
pub use with_generic_array::GenericRingBuffer;

/// Used internally. Computes the bitmask used to properly wrap the ringbuffers.
#[inline]
fn mask<T: 'static>(this: &impl RingBuffer<T>, index: usize) -> usize {
    index & (this.capacity() - 1)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;

    #[cfg(feature = "const_generics")]
    use crate::ConstGenericRingBuffer;
    use crate::{typenum, AllocRingBuffer, GenericRingBuffer, ReadWriteRingbuffer, RingBufferExt};

    #[test]
    fn run_test_default() {
        fn test_default(b: impl ReadWriteRingbuffer<i32>) {
            assert_eq!(b.capacity(), 8);
            assert_eq!(b.len(), 0);
        }

        test_default(AllocRingBuffer::with_capacity(8));
        test_default(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_default(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn run_test_new() {
        fn test_new(b: impl ReadWriteRingbuffer<i32>) {
            assert_eq!(b.capacity(), 8);
            assert_eq!(b.len(), 0);
        }

        test_new(AllocRingBuffer::with_capacity(8));
        test_new(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_new(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn test_default_eq_new() {
        assert_eq!(
            AllocRingBuffer::<i32>::default(),
            AllocRingBuffer::<i32>::new()
        );
        assert_eq!(
            GenericRingBuffer::<i32, typenum::U8>::default(),
            GenericRingBuffer::<i32, typenum::U8>::new()
        );
        #[cfg(feature = "const_generics")]
        assert_eq!(
            ConstGenericRingBuffer::<i32, 8>::default(),
            ConstGenericRingBuffer::<i32, 8>::new()
        );
    }

    #[test]
    fn run_test_len() {
        fn test_len(mut b: impl ReadWriteRingbuffer<i32>) {
            assert_eq!(0, b.len());
            b.push(1).unwrap();
            assert_eq!(1, b.len());
            b.push(2).unwrap();
            assert_eq!(2, b.len())
        }

        test_len(AllocRingBuffer::with_capacity(8));
        test_len(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_len(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn run_test_len_wrap() {
        fn test_len_wrap(mut b: impl RingBufferExt<i32>) {
            assert_eq!(0, b.len());
            b.push(1).unwrap();
            assert_eq!(1, b.len());
            b.push(2).unwrap();
            assert_eq!(2, b.len());

            // Now we are wrapping
            b.push_force(3);
            assert_eq!(2, b.len());
            b.push_force(4);
            assert_eq!(2, b.len());
        }

        test_len_wrap(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_len_wrap(ConstGenericRingBuffer::<i32, 2>::new());
        test_len_wrap(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_clear() {
        fn test_clear(mut b: impl ReadWriteRingbuffer<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            b.push(3).unwrap();

            b.clear();
            assert!(b.is_empty());
            assert_eq!(0, b.len());
        }

        test_clear(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_clear(ConstGenericRingBuffer::<i32, 8>::new());
        test_clear(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_empty() {
        fn test_empty(mut b: impl ReadWriteRingbuffer<i32>) {
            assert!(b.is_empty());
            b.push(1).unwrap();
            b.push(2).unwrap();
            b.push(3).unwrap();
            assert_ne!(b.is_empty(), true);

            b.clear();
            assert!(b.is_empty());
            assert_eq!(0, b.len());
        }

        test_empty(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_empty(ConstGenericRingBuffer::<i32, 8>::new());
        test_empty(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_iter() {
        fn test_iter(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            b.push(3).unwrap();

            let mut iter = b.iter();
            assert_eq!(&1, iter.next().unwrap());
            assert_eq!(&2, iter.next().unwrap());
            assert_eq!(&3, iter.next().unwrap());
        }

        test_iter(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_iter(ConstGenericRingBuffer::<i32, 8>::new());
        test_iter(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_double_iter() {
        fn test_double_iter(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            b.push(3).unwrap();

            let mut iter1 = b.iter();
            let mut iter2 = b.iter();

            assert_eq!(&1, iter1.next().unwrap());
            assert_eq!(&2, iter1.next().unwrap());
            assert_eq!(&3, iter1.next().unwrap());
            assert_eq!(&1, iter2.next().unwrap());
            assert_eq!(&2, iter2.next().unwrap());
            assert_eq!(&3, iter2.next().unwrap());
        }

        test_double_iter(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_double_iter(ConstGenericRingBuffer::<i32, 8>::new());
        test_double_iter(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_iter_wrap() {
        fn test_iter_wrap(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            // Wrap
            b.push_force(3);

            let mut iter = b.iter();
            assert_eq!(&2, iter.next().unwrap());
            assert_eq!(&3, iter.next().unwrap());
        }

        test_iter_wrap(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_iter_wrap(ConstGenericRingBuffer::<i32, 2>::new());
        test_iter_wrap(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_iter_mut() {
        fn test_iter_mut(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            b.push(3).unwrap();

            let mut i = b.iter_mut();
            while let Some(el) = i.next() {
                *el += 1;
            }

            assert_eq!(vec![2, 3, 4], b.to_vec())
        }

        test_iter_mut(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_iter_mut(ConstGenericRingBuffer::<i32, 8>::new());
        test_iter_mut(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_iter_mut_wrap() {
        fn test_iter_mut_wrap(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            b.push_force(3);

            let mut i = b.iter_mut();
            while let Some(el) = i.next() {
                *el += 1;
            }

            assert_eq!(vec![3, 4], b.to_vec())
        }

        test_iter_mut_wrap(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_iter_mut_wrap(ConstGenericRingBuffer::<i32, 2>::new());
        test_iter_mut_wrap(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_to_vec() {
        fn test_to_vec(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            b.push(3).unwrap();

            assert_eq!(vec![1, 2, 3], b.to_vec())
        }

        test_to_vec(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_to_vec(ConstGenericRingBuffer::<i32, 8>::new());
        test_to_vec(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_to_vec_wrap() {
        fn test_to_vec_wrap(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            // Wrap
            b.push_force(3);

            assert_eq!(vec![2, 3], b.to_vec())
        }

        test_to_vec_wrap(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_to_vec_wrap(ConstGenericRingBuffer::<i32, 2>::new());
        test_to_vec_wrap(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_index() {
        fn test_index(mut b: impl RingBufferExt<i32>) {
            b.push(2).unwrap();
            assert_eq!(b[0], 2)
        }

        test_index(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_index(ConstGenericRingBuffer::<i32, 8>::new());
        test_index(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_index_mut() {
        fn test_index_mut(mut b: impl RingBufferExt<i32>) {
            b.push(2).unwrap();

            assert_eq!(b[0], 2);

            b[0] = 5;

            assert_eq!(b[0], 5);
        }

        test_index_mut(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_index_mut(ConstGenericRingBuffer::<i32, 8>::new());
        test_index_mut(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_get_relative() {
        fn test_get_relative(mut b: impl RingBufferExt<i32>) {
            b.push(0).unwrap();
            b.push(1).unwrap();

            assert_eq!(b.get(0).unwrap(), &0);
            assert_eq!(b.get(1).unwrap(), &1);

            assert_eq!(b.get(2), None);
        }

        test_get_relative(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_get_relative(ConstGenericRingBuffer::<i32, 8>::new());
        test_get_relative(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_wrapping_get_relative() {
        fn test_wrapping_get_relative(mut b: impl RingBufferExt<i32>) {
            b.push(0).unwrap();
            b.push(1).unwrap();
            b.push_force(2);

            assert_eq!(b.get(0).unwrap(), &1);
            assert_eq!(b.get(1).unwrap(), &2);
        }

        test_wrapping_get_relative(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_wrapping_get_relative(ConstGenericRingBuffer::<i32, 2>::new());
        test_wrapping_get_relative(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_get_relative_zero_length() {
        fn test_get_relative_zero_length(b: impl RingBufferExt<i32>) {
            assert!(b.get(1).is_none());
        }

        test_get_relative_zero_length(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_get_relative_zero_length(ConstGenericRingBuffer::<i32, 8>::new());
        test_get_relative_zero_length(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_get_relative_mut() {
        fn test_get_relative_mut(mut b: impl RingBufferExt<i32>) {
            b.push(0).unwrap();
            b.push(1).unwrap();

            *b.get_mut(0).unwrap() = 3;
            *b.get_mut(1).unwrap() = 4;

            assert_eq!(b.get(0).unwrap(), &3);
            assert_eq!(b.get(1).unwrap(), &4);
        }

        test_get_relative_mut(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_get_relative_mut(ConstGenericRingBuffer::<i32, 8>::new());
        test_get_relative_mut(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_wrapping_get_relative_mut() {
        fn test_wrapping_get_relative_mut(mut b: impl RingBufferExt<i32>) {
            b.push(0).unwrap();
            b.push(1).unwrap();
            b.push_force(2);

            *b.get_mut(0).unwrap() = 3;

            assert_eq!(b.get(0).unwrap(), &3);
            assert_eq!(b.get(1).unwrap(), &2);
        }

        test_wrapping_get_relative_mut(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_wrapping_get_relative_mut(ConstGenericRingBuffer::<i32, 2>::new());
        test_wrapping_get_relative_mut(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_get_relative_mut_zero_length() {
        fn test_get_relative_mut_zero_length(mut b: impl RingBufferExt<i32>) {
            assert!(b.get_mut(1).is_none());
        }

        test_get_relative_mut_zero_length(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_get_relative_mut_zero_length(ConstGenericRingBuffer::<i32, 8>::new());
        test_get_relative_mut_zero_length(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_from_iterator() {
        fn test_from_iterator<T: RingBufferExt<i32>>() {
            let b: T = std::iter::repeat(1).take(1024).collect();
            assert_eq!(b.len(), 1024);
            assert_eq!(b.to_vec(), vec![1; 1024])
        }

        test_from_iterator::<AllocRingBuffer<i32>>();
        #[cfg(feature = "const_generics")]
        test_from_iterator::<ConstGenericRingBuffer<i32, 1024>>();
        test_from_iterator::<GenericRingBuffer<i32, typenum::U1024>>();
    }

    #[test]
    fn run_test_from_iterator_wrap() {
        fn test_from_iterator_wrap<T: RingBufferExt<i32>>() {
            let b: T = std::iter::repeat(1).take(8000).collect();
            assert_eq!(b.len(), b.capacity());
            assert_eq!(b.to_vec(), vec![1; b.capacity()])
        }

        test_from_iterator_wrap::<AllocRingBuffer<i32>>();
        #[cfg(feature = "const_generics")]
        test_from_iterator_wrap::<ConstGenericRingBuffer<i32, 1024>>();
        test_from_iterator_wrap::<GenericRingBuffer<i32, typenum::U1024>>();
    }

    #[test]
    fn run_test_contains() {
        fn test_contains(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();

            assert!(b.contains(&1));
            assert!(b.contains(&2));
        }

        test_contains(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_contains(ConstGenericRingBuffer::<i32, 8>::new());
        test_contains(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_is_full() {
        fn test_is_full(mut b: impl ReadWriteRingbuffer<i32>) {
            assert!(!b.is_full());
            b.push(1).unwrap();
            assert!(!b.is_full());
            b.push(2).unwrap();
            assert!(b.is_full());
        }

        test_is_full(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_is_full(ConstGenericRingBuffer::<i32, 2>::new());
        test_is_full(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_front_some() {
        fn test_front_some(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();

            assert_eq!(b.front(), Some(&2));
        }

        test_front_some(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_front_some(ConstGenericRingBuffer::<i32, 2>::new());
        test_front_some(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_front_none() {
        fn test_front_none(b: impl RingBufferExt<i32>) {
            assert_eq!(b.front(), None);
        }

        test_front_none(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_front_none(ConstGenericRingBuffer::<i32, 8>::new());
        test_front_none(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_back_some() {
        fn test_back_some(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();

            assert_eq!(b.back(), Some(&1));
        }

        test_back_some(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_back_some(ConstGenericRingBuffer::<i32, 2>::new());
        test_back_some(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_back_none() {
        fn test_back_none(b: impl RingBufferExt<i32>) {
            assert_eq!(b.back(), None);
        }

        test_back_none(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_back_none(ConstGenericRingBuffer::<i32, 8>::new());
        test_back_none(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_front_some_mut() {
        fn test_front_some_mut(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();

            assert_eq!(b.front_mut(), Some(&mut 2));
        }

        test_front_some_mut(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_front_some_mut(ConstGenericRingBuffer::<i32, 2>::new());
        test_front_some_mut(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_front_none_mut() {
        fn test_front_none_mut(mut b: impl RingBufferExt<i32>) {
            assert_eq!(b.front_mut(), None);
        }

        test_front_none_mut(AllocRingBuffer::with_capacity(8));
        #[cfg(feature = "const_generics")]
        test_front_none_mut(ConstGenericRingBuffer::<i32, 8>::new());
        test_front_none_mut(GenericRingBuffer::<i32, typenum::U8>::new());
    }

    #[test]
    fn run_test_back_some_mut() {
        fn test_back_some_mut(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();

            assert_eq!(b.back_mut(), Some(&mut 1));
        }

        test_back_some_mut(AllocRingBuffer::with_capacity(2));
        #[cfg(feature = "const_generics")]
        test_back_some_mut(ConstGenericRingBuffer::<i32, 2>::new());
        test_back_some_mut(GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_back_none_mut() {
        fn test_back_none_mut(mut b: impl RingBufferExt<i32>) {
            assert_eq!(b.back_mut(), None);
        }

        test_back_none_mut(AllocRingBuffer::with_capacity(8));
        test_back_none_mut(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_back_none_mut(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn run_test_dequeue() {
        fn test_pop(mut b: impl RingBufferExt<i32>) {
            b.push(0).unwrap();
            b.push(1).unwrap();

            assert_eq!(b.len(), 2);

            assert_eq!(b.pop(), Some(0));
            assert_eq!(b.pop(), Some(1));

            assert_eq!(b.len(), 0);

            assert_eq!(b.pop(), None);
        }

        test_pop(AllocRingBuffer::with_capacity(8));
        test_pop(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_pop(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn run_test_skip() {
        fn test_skip(mut b: impl ReadWriteRingbuffer<i32>) {
            b.push(0).unwrap();
            b.push(1).unwrap();

            assert_eq!(b.len(), 2);

            b.skip();
            b.skip();

            assert_eq!(b.len(), 0)
        }

        test_skip(AllocRingBuffer::with_capacity(8));
        test_skip(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_skip(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn run_test_push_pop_push() {
        fn test_push_pop_push(mut b: impl RingBufferExt<i32>) {
            b.push(0).unwrap();
            b.push(1).unwrap();

            assert_eq!(b.pop(), Some(0));
            assert_eq!(b.pop(), Some(1));
            assert_eq!(b.pop(), None);

            b.push_force(0);
            b.push_force(1);

            assert_eq!(b.pop(), Some(0));
            assert_eq!(b.pop(), Some(1));
            assert_eq!(b.pop(), None);
        }

        test_push_pop_push(AllocRingBuffer::with_capacity(8));
        test_push_pop_push(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_push_pop_push(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn run_test_push_pop_push_full() {
        fn test_push_pop_push_full(mut b: impl RingBufferExt<i32>) {
            b.push_force(0);
            b.push_force(1);
            b.push_force(2);

            assert_eq!(b.pop(), Some(1));
            assert_eq!(b.pop(), Some(2));
            assert_eq!(b.pop(), None);

            b.push_force(0);
            b.push_force(1);
            b.push_force(2);

            assert_eq!(b.pop(), Some(1));
            assert_eq!(b.pop(), Some(2));
            assert_eq!(b.pop(), None);
        }

        test_push_pop_push_full(AllocRingBuffer::with_capacity(2));
        test_push_pop_push_full(GenericRingBuffer::<i32, typenum::U2>::new());
        #[cfg(feature = "const_generics")]
        test_push_pop_push_full(ConstGenericRingBuffer::<i32, 2>::new());
    }

    #[test]
    fn run_test_push_pop_push_full_get() {
        fn test_push_pop_push_full_get(mut b: impl RingBufferExt<i32>) {
            b.push_force(0);
            b.push_force(1);
            b.push_force(2);

            assert_eq!(b.pop(), Some(1));
            assert_eq!(b.pop(), Some(2));
            assert_eq!(b.pop(), None);

            b.push_force(0);
            b.push_force(1);
            b.push_force(2);

            assert_eq!(b.pop(), Some(1));
            assert_eq!(b.pop(), Some(2));
            assert_eq!(b.pop(), None);

            b.push_force(0);
            b.push_force(1);
            b.push_force(2);

            assert_eq!(b.get(0), Some(&1));
            assert_eq!(b.get(1), Some(&2));
        }

        test_push_pop_push_full_get(AllocRingBuffer::with_capacity(2));
        test_push_pop_push_full_get(GenericRingBuffer::<i32, typenum::U2>::new());
        #[cfg(feature = "const_generics")]
        test_push_pop_push_full_get(ConstGenericRingBuffer::<i32, 2>::new());
    }

    #[test]
    fn run_test_push_pop_push_full_get_rep() {
        fn test_push_pop_push_full_get_rep(mut rb: impl RingBufferExt<i32>) {
            for _ in 0..100_000 {
                rb.push_force(1);
                rb.push_force(2);

                assert_eq!(rb.pop(), Some(1));
                assert_eq!(rb.pop(), Some(2));

                rb.push_force(1);
                rb.push_force(2);

                assert_eq!(rb.pop(), Some(1));
                assert_eq!(rb.pop(), Some(2));

                rb.push_force(1);
                rb.push_force(2);

                assert_eq!(rb.get(0), Some(&1));
                assert_eq!(rb.get(1), Some(&2));
            }
        }

        test_push_pop_push_full_get_rep(AllocRingBuffer::with_capacity(8));
        test_push_pop_push_full_get_rep(GenericRingBuffer::<i32, typenum::U8>::new());
        #[cfg(feature = "const_generics")]
        test_push_pop_push_full_get_rep(ConstGenericRingBuffer::<i32, 8>::new());
    }

    #[test]
    fn run_test_try_push() {
        fn test_try_push(mut b: impl RingBufferExt<i32>) {
            b.push(1).unwrap();
            b.push(2).unwrap();
            assert_eq!(b.push(3), Err(3));
            assert!(b.is_full());
            assert_eq!(b.to_vec(), vec![1, 2]);
        }

        test_try_push(AllocRingBuffer::with_capacity(2));
        test_try_push(GenericRingBuffer::<i32, typenum::U2>::new());
        #[cfg(feature = "const_generics")]
        test_try_push(ConstGenericRingBuffer::<i32, 2>::new());
    }
}

#[cfg(test)]
mod test_dropping {
    extern crate std;

    use std::boxed::Box;
    use std::cell::{RefCell, RefMut};

    use crate::{typenum, AllocRingBuffer, GenericRingBuffer, RingBufferExt, WritableRingbuffer};

    #[cfg(feature = "const_generics")]
    use crate::ConstGenericRingBuffer;

    struct DropTest {
        flag: bool,
    }

    struct Dropee<'a> {
        parent: Option<RefMut<'a, DropTest>>,
    }

    impl<'a> Drop for Dropee<'a> {
        fn drop(&mut self) {
            if let Some(parent) = &mut self.parent {
                parent.flag = true;
            }
        }
    }

    macro_rules! test_dropped {
            ($constructor: block) => {
                {
                    let dt = Box::leak(Box::new(RefCell::new(DropTest { flag: false })));
                    {
                        let d = Dropee { parent: Some(dt.borrow_mut()) };
                        let mut rb = { $constructor };
                        assert!(rb.push(d).is_ok());
                        rb.push_force(Dropee { parent: None })
                    }
                    assert!(dt.borrow_mut().flag);
                    unsafe {
                        // SAFETY: we know Dropee, which needed the static lifetime, has been dropped (by the assert)
                        // we could probably skip this, but this makes sure we don't leak any memory
                        let ptr: *mut RefCell<DropTest> = std::mem::transmute::<&RefCell<DropTest>, _>(dt);
                        drop(Box::from_raw(ptr));
                    }
                }
            };
        }

    #[test]
    fn run_test_drops_contents_alloc() {
        test_dropped!({ AllocRingBuffer::with_capacity(1) });
    }

    #[test]
    fn run_test_drops_contents_generic() {
        test_dropped!({ GenericRingBuffer::<_, typenum::U1>::new() });
    }

    #[cfg(feature = "const_generics")]
    #[test]
    fn run_test_drops_contents_const_generic() {
        test_dropped!({ ConstGenericRingBuffer::<_, 1>::new() });
    }
}
