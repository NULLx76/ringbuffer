#![no_std]
#![cfg_attr(feature = "const_generics", feature(const_generics))]
#![cfg_attr(feature = "const_generics", allow(incomplete_features))]
#![deny(missing_docs)]
//! # Ringbuffer
//! [![Drone (self-hosted)](https://img.shields.io/drone/build/0x76/ringbuffer?logo=drone&server=https%3A%2F%2Fdrone.xirion.net&style=for-the-badge)](https://drone.xirion.net/0x76/ringbuffer)
//! [![Codecov](https://img.shields.io/codecov/c/github/NULLx76/ringbuffer?logo=codecov&style=for-the-badge)](https://codecov.io/gh/NULLx76/ringbuffer)
//! [![Docs.rs](https://img.shields.io/badge/docs.rs-ringbuffer-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/ringbuffer)
//! [![Crates.io](https://img.shields.io/crates/v/ringbuffer?logo=rust&style=for-the-badge)](https://crates.io/crates/ringbuffer)
//!
//! The ringbuffer crate provides safe fixed size circular buffers (ringbuffers) in rust.
//!
//! Implementations for three kinds of ringbuffers, with a very similar API are provided:
//!
//! | type | description |
//! | --- | --- |
//! | [`AllocRingBuffer`] | Ringbuffer allocated on the heap at runtime. This ringbuffer is still fixed size and requires alloc. |
//! | [`GenericRingBuffer`] | Ringbuffer allocated on the stack. This is done using the `typenum` crate to provide a const-generic like interface without needing nightly |
//! | [`ConstGenericRingBuffer`] | Ringbuffer which uses const generics to allocate on the stack. This type is feature-gated behind `const_generics` and only works in nightly rust.|
//!
//! # Usage
//!
//! ```rust
//! # use ringbuffer::AllocRingBuffer;
//! # use ringbuffer::RingBuffer;
//!
//! let mut buffer = AllocRingBuffer::with_capacity(2);
//!
//! // First entry of the buffer is now 5.
//! buffer.push(5);
//!
//! assert_eq!(buffer.get_absolute(0), Some(&5));
//!
//! // Second entry is now 42.
//! buffer.push(42);
//!
//! // Because capacity is reached the next push will be the first item of the buffer.
//! buffer.push(1);
//! assert_eq!(buffer.get_absolute(0), Some(&1));
//!
//! ```
//!
//! # Features
//!
//! | name | default | description |
//! | --- | --- | --- |
//! | alloc | ✓ | |
//! | const_generics | ✗ | Enables the ConstGenericRingBuffer. This requires nightly. |
//! | generic_uninit | ✗  | Enables the unsafe `new_uninit` function on [`GenericRingBuffer`] and [`ConstGenericRingBuffer`] used for faster initialization |
//! | generic_array | ✓ | Enabled by default. Disable this feature to remove the `generic_array` and `typenum` dependencies (also disables GenericRingBuffer). |
//!
//! # License
//!
//! Licensed under GNU Lesser General Public License v3.0

#[macro_use]
pub(crate) mod ringbuffer_trait;
pub use ringbuffer_trait::RingBuffer;

#[cfg(feature = "alloc")]
mod with_alloc;
#[cfg(feature = "alloc")]
pub use with_alloc::AllocRingBuffer;

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

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;

    use std::vec;

    #[test]
    fn run_test_default() {
        fn test_default(b: impl RingBuffer<i32>) {
            assert_eq!(b.capacity(), 10);
            assert_eq!(b.len(), 0);
        }

        test_default(with_alloc::AllocRingBuffer::with_capacity(10));
        test_default(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_default(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_new() {
        fn test_new(b: impl RingBuffer<i32>) {
            assert_eq!(b.capacity(), 10);
            assert_eq!(b.len(), 0);
        }

        test_new(with_alloc::AllocRingBuffer::with_capacity(10));
        test_new(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_new(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn test_default_eq_new() {
        assert_eq!(
            with_alloc::AllocRingBuffer::<i32>::default(),
            with_alloc::AllocRingBuffer::<i32>::new()
        );
        assert_eq!(
            with_const_generics::ConstGenericRingBuffer::<i32, 10>::default(),
            with_const_generics::ConstGenericRingBuffer::<i32, 10>::new()
        );
        assert_eq!(
            with_generic_array::GenericRingBuffer::<i32, typenum::U10>::default(),
            with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new()
        );
    }

    #[test]
    fn run_test_len() {
        fn test_len(mut b: impl RingBuffer<i32>) {
            assert_eq!(0, b.len());
            b.push(1);
            assert_eq!(1, b.len());
            b.push(2);
            assert_eq!(2, b.len())
        }

        test_len(with_alloc::AllocRingBuffer::with_capacity(10));
        test_len(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_len(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_len_wrap() {
        fn test_len_wrap(mut b: impl RingBuffer<i32>) {
            assert_eq!(0, b.len());
            b.push(1);
            assert_eq!(1, b.len());
            b.push(2);
            assert_eq!(2, b.len());
            // Now we are wrapping
            b.push(3);
            assert_eq!(2, b.len());
            b.push(4);
            assert_eq!(2, b.len());
        }

        test_len_wrap(with_alloc::AllocRingBuffer::with_capacity(2));
        test_len_wrap(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_len_wrap(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_clear() {
        fn test_clear(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            b.push(3);

            b.clear();
            assert!(b.is_empty());
            assert_eq!(0, b.len());
        }

        test_clear(with_alloc::AllocRingBuffer::with_capacity(10));
        test_clear(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_clear(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_empty() {
        fn test_empty(mut b: impl RingBuffer<i32>) {
            assert!(b.is_empty());
            b.push(1);
            b.push(2);
            b.push(3);
            assert_ne!(b.is_empty(), true);

            b.clear();
            assert!(b.is_empty());
            assert_eq!(0, b.len());
        }

        test_empty(with_alloc::AllocRingBuffer::with_capacity(10));
        test_empty(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_empty(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_iter() {
        fn test_iter(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            b.push(3);

            let mut iter = b.iter();
            assert_eq!(&1, iter.next().unwrap());
            assert_eq!(&2, iter.next().unwrap());
            assert_eq!(&3, iter.next().unwrap());
        }

        test_iter(with_alloc::AllocRingBuffer::with_capacity(10));
        test_iter(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_iter(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_double_iter() {
        fn test_double_iter(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            b.push(3);

            let mut iter1 = b.iter();
            let mut iter2 = b.iter();

            assert_eq!(&1, iter1.next().unwrap());
            assert_eq!(&2, iter1.next().unwrap());
            assert_eq!(&3, iter1.next().unwrap());
            assert_eq!(&1, iter2.next().unwrap());
            assert_eq!(&2, iter2.next().unwrap());
            assert_eq!(&3, iter2.next().unwrap());
        }

        test_double_iter(with_alloc::AllocRingBuffer::with_capacity(10));
        test_double_iter(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_double_iter(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_iter_wrap() {
        fn test_iter_wrap(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            // Wrap
            b.push(3);

            let mut iter = b.iter();
            assert_eq!(&2, iter.next().unwrap());
            assert_eq!(&3, iter.next().unwrap());
        }

        test_iter_wrap(with_alloc::AllocRingBuffer::with_capacity(2));
        test_iter_wrap(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_iter_wrap(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_iter_mut() {
        fn test_iter_mut(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            b.push(3);

            for el in b.iter_mut() {
                *el += 1;
            }

            assert_eq!(vec![2, 3, 4], b.to_vec())
        }

        test_iter_mut(with_alloc::AllocRingBuffer::with_capacity(10));
        test_iter_mut(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_iter_mut(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_iter_mut_wrap() {
        fn test_iter_mut_wrap(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            b.push(3);

            for el in b.iter_mut() {
                *el += 1;
            }

            assert_eq!(vec![3, 4], b.to_vec())
        }

        test_iter_mut_wrap(with_alloc::AllocRingBuffer::with_capacity(2));
        test_iter_mut_wrap(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_iter_mut_wrap(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_to_vec() {
        fn test_to_vec(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            b.push(3);

            assert_eq!(vec![1, 2, 3], b.to_vec())
        }

        test_to_vec(with_alloc::AllocRingBuffer::with_capacity(10));
        test_to_vec(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_to_vec(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_to_vec_wrap() {
        fn test_to_vec_wrap(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);
            // Wrap
            b.push(3);

            assert_eq!(vec![2, 3], b.to_vec())
        }

        test_to_vec_wrap(with_alloc::AllocRingBuffer::with_capacity(2));
        test_to_vec_wrap(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_to_vec_wrap(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_index() {
        fn test_index(mut b: impl RingBuffer<i32>) {
            b.push(2);
            assert_eq!(b[0], 2)
        }

        test_index(with_alloc::AllocRingBuffer::with_capacity(10));
        test_index(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_index(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_index_mut() {
        fn test_index_mut(mut b: impl RingBuffer<i32>) {
            b.push(2);

            assert_eq!(b[0], 2);

            b[0] = 5;

            assert_eq!(b[0], 5);
        }

        test_index_mut(with_alloc::AllocRingBuffer::with_capacity(10));
        test_index_mut(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_index_mut(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_peek_some() {
        fn test_peek_some(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);

            assert_eq!(b.peek(), Some(&1));
        }

        test_peek_some(with_alloc::AllocRingBuffer::with_capacity(2));
        test_peek_some(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_peek_some(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_peek_none() {
        fn test_peek_none(b: impl RingBuffer<i32>) {
            assert_eq!(b.peek(), None);
        }

        test_peek_none(with_alloc::AllocRingBuffer::with_capacity(10));
        test_peek_none(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_peek_none(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_get_relative() {
        fn test_get_relative(mut b: impl RingBuffer<i32>) {
            b.push(0);
            b.push(1);

            // [0, ...]
            //      ^
            // [0, 1, ...]
            //         ^
            // get[(index + 0) % len] = 0 (wrap to 0 because len == 2)
            // get[(index + 1) % len] = 1
            assert_eq!(b.get(0).unwrap(), &0);
            assert_eq!(b.get(1).unwrap(), &1);

            // TODO: Is this intended behaviour?
            assert_eq!(b.get(2).unwrap(), &0);
            assert_eq!(b.get(3).unwrap(), &1);
        }

        test_get_relative(with_alloc::AllocRingBuffer::with_capacity(10));
        test_get_relative(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_get_relative(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_wrapping_get_relative() {
        fn test_wrapping_get_relative(mut b: impl RingBuffer<i32>) {
            b.push(0);
            b.push(1);
            b.push(2);

            // [0, ...]
            //      ^
            // [0, 1]
            //  ^
            // [2, 1]
            //     ^
            // get(0) == b[index] = 1
            // get(1) == b[(index+1) % len] = 1
            assert_eq!(b.get(0).unwrap(), &1);
            assert_eq!(b.get(1).unwrap(), &2);
        }

        test_wrapping_get_relative(with_alloc::AllocRingBuffer::with_capacity(2));
        test_wrapping_get_relative(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_wrapping_get_relative(
            with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new(),
        );
    }

    #[test]
    fn run_test_get_relative_zero_length() {
        fn test_get_relative_zero_length(b: impl RingBuffer<i32>) {
            assert!(b.get(1).is_none());
        }

        test_get_relative_zero_length(with_alloc::AllocRingBuffer::with_capacity(10));
        test_get_relative_zero_length(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_get_relative_zero_length(
            with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new(),
        );
    }

    #[test]
    fn run_test_get_relative_mut() {
        fn test_get_relative_mut(mut b: impl RingBuffer<i32>) {
            b.push(0);
            b.push(1);

            // [0, ...]
            //      ^
            // [0, 1, ...]
            //         ^
            // get[(index + 0) % len] = 0 (wrap to 0 because len == 2)
            // get[(index + 1) % len] = 1
            *b.get_mut(0).unwrap() = 3;
            *b.get_mut(1).unwrap() = 4;

            assert_eq!(b.get(0).unwrap(), &3);
            assert_eq!(b.get(1).unwrap(), &4);
        }

        test_get_relative_mut(with_alloc::AllocRingBuffer::with_capacity(10));
        test_get_relative_mut(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_get_relative_mut(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_wrapping_get_relative_mut() {
        fn test_wrapping_get_relative_mut(mut b: impl RingBuffer<i32>) {
            b.push(0);
            b.push(1);
            b.push(2);

            *b.get_mut(0).unwrap() = 3;

            // [0, ...]
            //      ^
            // [0, 1]
            //  ^
            // [2, 1]
            //     ^
            // get(0) == b[index] = 1
            // get(1) == b[(index+1) % len] = 1
            assert_eq!(b.get(0).unwrap(), &3);
            assert_eq!(b.get(1).unwrap(), &2);
        }

        test_wrapping_get_relative_mut(with_alloc::AllocRingBuffer::with_capacity(2));
        test_wrapping_get_relative_mut(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_wrapping_get_relative_mut(
            with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new(),
        );
    }

    #[test]
    fn run_test_get_relative_mut_zero_length() {
        fn test_get_relative_mut_zero_length(mut b: impl RingBuffer<i32>) {
            assert!(b.get_mut(1).is_none());
        }

        test_get_relative_mut_zero_length(with_alloc::AllocRingBuffer::with_capacity(10));
        test_get_relative_mut_zero_length(
            with_const_generics::ConstGenericRingBuffer::<i32, 10>::new(),
        );
        test_get_relative_mut_zero_length(
            with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new(),
        );
    }

    #[test]
    fn run_test_get_absolute() {
        fn test_get_absolute(mut b: impl RingBuffer<i32>) {
            b.push(0);
            b.push(1);

            // [0, ...]
            //      ^
            // [0, 1, ...]
            //         ^
            // get[0] = 0
            // get[1] = 1
            assert_eq!(b.get_absolute(0).unwrap(), &0);
            assert_eq!(b.get_absolute(1).unwrap(), &1);
            assert!(b.get_absolute(2).is_none());
        }

        test_get_absolute(with_alloc::AllocRingBuffer::with_capacity(10));
        test_get_absolute(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_get_absolute(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_from_iterator() {
        fn test_from_iterator<T: RingBuffer<i32>>() {
            let b: T = std::iter::repeat(1).take(100).collect();
            assert_eq!(b.len(), 100);
            assert_eq!(b.to_vec(), vec![1; 100])
        }

        test_from_iterator::<AllocRingBuffer<i32>>();
        test_from_iterator::<ConstGenericRingBuffer<i32, 1024>>();
        test_from_iterator::<GenericRingBuffer<i32, typenum::U1024>>();
    }

    #[test]
    fn run_test_from_iterator_wrap() {
        fn test_from_iterator_wrap<T: RingBuffer<i32>>() {
            fn test_from_iterator<T: RingBuffer<i32>>() {
                let b: T = std::iter::repeat(1).take(10000).collect();
                assert_eq!(b.len(), b.capacity());
                assert_eq!(b.to_vec(), vec![1; b.capacity()])
            }

            test_from_iterator::<AllocRingBuffer<i32>>();
            test_from_iterator::<ConstGenericRingBuffer<i32, 1024>>();
            test_from_iterator::<GenericRingBuffer<i32, typenum::U1024>>();
        }

        test_from_iterator_wrap::<AllocRingBuffer<i32>>();
        test_from_iterator_wrap::<ConstGenericRingBuffer<i32, 1024>>();
        test_from_iterator_wrap::<GenericRingBuffer<i32, typenum::U1024>>();
    }

    #[test]
    fn run_test_get_relative_negative() {
        fn test_get_relative_negative(mut b: impl RingBuffer<i32>) {
            b.push(0);
            b.push(1);

            // [0, ...]
            //      ^
            // [0, 1, ...]
            //         ^
            // get[(index + -1) % len] = 1
            // get[(index + -2) % len] = 0 (wrap to 1 because len == 2)
            assert_eq!(b.get(-1).unwrap(), &1);
            assert_eq!(b.get(-2).unwrap(), &0);

            // TODO: Is this intended behaviour?
            assert_eq!(b.get(-3).unwrap(), &1);
            assert_eq!(b.get(-4).unwrap(), &0);
        }

        test_get_relative_negative(with_alloc::AllocRingBuffer::with_capacity(10));
        test_get_relative_negative(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_get_relative_negative(
            with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new(),
        );
    }

    #[test]
    fn run_test_contains() {
        fn test_contains(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);

            assert!(b.contains(&1));
            assert!(b.contains(&2));
        }

        test_contains(with_alloc::AllocRingBuffer::with_capacity(10));
        test_contains(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_contains(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_is_full() {
        fn test_is_full(mut b: impl RingBuffer<i32>) {
            assert!(!b.is_full());
            b.push(1);
            assert!(!b.is_full());
            b.push(2);
            assert!(b.is_full());
        }

        test_is_full(with_alloc::AllocRingBuffer::with_capacity(2));
        test_is_full(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_is_full(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_front_some() {
        fn test_front_some(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);

            assert_eq!(b.front(), Some(&1));
        }

        test_front_some(with_alloc::AllocRingBuffer::with_capacity(2));
        test_front_some(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_front_some(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_front_none() {
        fn test_front_none(b: impl RingBuffer<i32>) {
            assert_eq!(b.front(), None);
        }

        test_front_none(with_alloc::AllocRingBuffer::with_capacity(10));
        test_front_none(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_front_none(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_back_some() {
        fn test_back_some(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);

            assert_eq!(b.back(), Some(&2));
        }

        test_back_some(with_alloc::AllocRingBuffer::with_capacity(2));
        test_back_some(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_back_some(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_back_none() {
        fn test_back_none(b: impl RingBuffer<i32>) {
            assert_eq!(b.back(), None);
        }

        test_back_none(with_alloc::AllocRingBuffer::with_capacity(10));
        test_back_none(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_back_none(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_front_some_mut() {
        fn test_front_some_mut(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);

            assert_eq!(b.front_mut(), Some(&mut 1));
        }

        test_front_some_mut(with_alloc::AllocRingBuffer::with_capacity(2));
        test_front_some_mut(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_front_some_mut(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_front_none_mut() {
        fn test_front_none_mut(mut b: impl RingBuffer<i32>) {
            assert_eq!(b.front_mut(), None);
        }

        test_front_none_mut(with_alloc::AllocRingBuffer::with_capacity(10));
        test_front_none_mut(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_front_none_mut(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }

    #[test]
    fn run_test_back_some_mut() {
        fn test_back_some_mut(mut b: impl RingBuffer<i32>) {
            b.push(1);
            b.push(2);

            assert_eq!(b.back_mut(), Some(&mut 2));
        }

        test_back_some_mut(with_alloc::AllocRingBuffer::with_capacity(2));
        test_back_some_mut(with_const_generics::ConstGenericRingBuffer::<i32, 2>::new());
        test_back_some_mut(with_generic_array::GenericRingBuffer::<i32, typenum::U2>::new());
    }

    #[test]
    fn run_test_back_none_mut() {
        fn test_back_none_mut(mut b: impl RingBuffer<i32>) {
            assert_eq!(b.back_mut(), None);
        }

        test_back_none_mut(with_alloc::AllocRingBuffer::with_capacity(10));
        test_back_none_mut(with_const_generics::ConstGenericRingBuffer::<i32, 10>::new());
        test_back_none_mut(with_generic_array::GenericRingBuffer::<i32, typenum::U10>::new());
    }
}
