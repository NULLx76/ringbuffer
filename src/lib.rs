#![no_std]


#[cfg(not(feature="alloc"))]
mod with_alloc;
#[cfg(not(feature="alloc"))]
pub use with_alloc::RingBuffer;


mod with_generic_array;
pub use with_generic_array::RingBuffer as GenericRingBuffer;
pub use generic_array::{ArrayLength, typenum};