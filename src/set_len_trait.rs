/// `SetLen` is a trait defining the unsafe `set_len` method
/// on ringbuffers that support the operation.
pub trait SetLen {
    /// Force the length of the ringbuffer to `new_len`.
    ///
    /// Note that downsizing will not call Drop on elements at `new_len..old_len`,
    /// potentially causing a memory leak.
    ///
    /// # Panics
    /// Panics if `new_len` is greater than the ringbuffer capacity.
    ///
    /// # Safety
    /// - Safe when `new_len <= old_len`.
    /// - Safe when `new_len > old_len` and all the elements at `old_len..new_len` are already initialized.
    unsafe fn set_len(&mut self, new_len: usize);
}

/// Implement `set_len` given a `readptr` and a `writeptr`.
#[macro_export]
macro_rules! impl_ring_buffer_set_len {
    ($readptr: ident, $writeptr: ident) => {
        #[inline]
        unsafe fn set_len(&mut self, new_len: usize) {
            let cap = self.capacity();
            assert!(new_len <= cap, "Cannot set the a length of {new_len} on a ringbuffer with capacity for {cap} items");
            self.$writeptr = self.$readptr + new_len;
        }
    };
}
