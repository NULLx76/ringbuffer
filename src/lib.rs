#![no_std]
#![cfg_attr(feature = "const_generics", feature(const_generics))]
#![cfg_attr(feature = "const_generics", allow(incomplete_features))]

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

pub(crate) mod ringbuffer_trait;
pub use ringbuffer_trait::RingBuffer;

