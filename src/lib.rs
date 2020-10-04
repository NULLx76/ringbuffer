#![no_std]
#![cfg_attr(feature = "const_generics", feature(const_generics))]
#![cfg_attr(feature = "const_generics", allow(incomplete_features))]

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
}
