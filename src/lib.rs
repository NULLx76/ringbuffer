#![no_std]
#![cfg_attr(feature = "const_generics", feature(const_generics))]
#![cfg_attr(feature = "const_generics", allow(incomplete_features))]

#[cfg(feature = "alloc")]
mod with_alloc;
#[cfg(feature = "alloc")]
pub use with_alloc::RingBuffer;

#[cfg(feature = "const_generics")]
mod with_const_generics;

mod with_generic_array;
pub use generic_array::{typenum, ArrayLength};
pub use with_generic_array::RingBuffer as GenericRingBuffer;
