#![no_std]

#[cfg(feature = "alloc")]
mod with_alloc;
#[cfg(eature = "alloc")]
pub use with_alloc::RingBuffer;

mod with_generic_array;
pub use generic_array::{typenum, ArrayLength};
pub use with_generic_array::RingBuffer as GenericRingBuffer;
