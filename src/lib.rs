#![no_std]

#[cfg(alloc)]
mod alloc;
#[cfg(alloc)]
pub use alloc::RingBuffer;


#[cfg(generic)]
mod generic;
#[cfg(generic)]
pub use generic::{RingBuffer as GenericRingBuffer, typenum};
#[cfg(generic)]
pub use generic_array::ArrayLength;