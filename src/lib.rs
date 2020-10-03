#![no_std]
#![cfg_attr(feature = "const_generics", feature(const_generics))]
#![cfg_attr(feature = "const_generics", allow(incomplete_features))]

#[cfg(feature = "alloc")]
mod with_alloc;
#[cfg(feature = "alloc")]
pub use with_alloc::RingBuffer;

#[cfg(feature = "const_generics")]
mod with_const_generics;
#[cfg(feature = "const_generics")]
pub use with_const_generics::RingBuffer as ConstGenericRingBuffer;

#[cfg(feature = "generic_array")]
mod with_generic_array;
#[cfg(feature = "generic_array")]
pub use generic_array::{typenum, ArrayLength};
#[cfg(feature = "generic_array")]
pub use with_generic_array::RingBuffer as GenericRingBuffer;
