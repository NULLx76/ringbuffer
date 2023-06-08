extern crate std;

use crate::RingBufferExt;
use crate::{AllocRingBuffer, ConstGenericRingBuffer, GrowableAllocRingBuffer};
use alloc::collections::{LinkedList, VecDeque};
use alloc::string::ToString;
use std::vec;

macro_rules! convert_test {
    ($name: ident: $from: expr => $to: ty) => {
        #[test]
        fn $name() {
            let a = $from;

            let b: $to = a.into();
            assert_eq!(b.to_vec(), vec!['1', '2']);
        }
    };
}

macro_rules! convert_tests {
    (
        [$($name: ident: $from: expr),* $(,)?]
        => $to: ty
    ) => {
        $(
            convert_test!($name: $from => $to);
        )*
    };
}

convert_tests!(
    [
        alloc_from_vec: vec!['1', '2'],
        alloc_from_ll: {let mut l = LinkedList::new(); l.push_back('1'); l.push_back('2'); l},
        alloc_from_vd: {let mut l = VecDeque::new(); l.push_back('1'); l.push_back('2'); l},
        alloc_from_str: "12".to_string(),
        alloc_from_str_slice: "12",
        alloc_from_slice: {let a: &[char] = &['1', '2']; a},
        alloc_from_const_slice: {let a: &[char; 2] = &['1', '2']; a},
        alloc_from_arr: {let a: [char; 2] = ['1', '2']; a},

        alloc_from_cgrb: {let a = ConstGenericRingBuffer::from(['1', '2']); a},
        alloc_from_garb: {let a = GrowableAllocRingBuffer::from(['1', '2']); a},
    ] => AllocRingBuffer::<_, _>
);

convert_tests!(
    [
        growable_alloc_from_vec: vec!['1', '2'],
        growable_alloc_from_ll: {let mut l = LinkedList::new(); l.push_back('1'); l.push_back('2'); l},
        growable_alloc_from_vd: {let mut l = VecDeque::new(); l.push_back('1'); l.push_back('2'); l},
        growable_alloc_from_str: "12".to_string(),
        growable_alloc_from_str_slice: "12",
        growable_alloc_from_slice: {let a: &[char] = &['1', '2']; a},
        growable_alloc_from_const_slice: {let a: &[char; 2] = &['1', '2']; a},
        growable_alloc_from_arr: {let a: [char; 2] = ['1', '2']; a},

        growable_alloc_from_cgrb: {let a = ConstGenericRingBuffer::from(['1', '2']); a},
        growable_alloc_from_arb: {let a = AllocRingBuffer::from(['1', '2']); a},
    ] => GrowableAllocRingBuffer::<_>
);

convert_tests!(
    [
        const_from_vec: vec!['1', '2'],
        const_from_ll: {let mut l = LinkedList::new(); l.push_back('1'); l.push_back('2'); l},
        const_from_vd: {let mut l = VecDeque::new(); l.push_back('1'); l.push_back('2'); l},
        const_from_str: "12".to_string(),
        const_from_str_slice: "12",
        const_from_slice: {let a: &[char] = &['1', '2']; a},
        const_from_const_slice: {let a: &[char; 2] = &['1', '2']; a},
        const_from_arr: {let a: [char; 2] = ['1', '2']; a},

        const_from_garb: {let a = GrowableAllocRingBuffer::from(['1', '2']); a},
        const_from_arb: {let a = AllocRingBuffer::from(['1', '2']); a},
    ] => ConstGenericRingBuffer::<_, 2>
);
