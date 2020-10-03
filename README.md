# ringbuffer
A fixed-size circular buffer written in Rust.

# Usage
```rust
use ringbuffer::RingBuffer;

let mut buffer = RingBuffer::with_capacity(2);

// First entry of the buffer is now 5.
buffer.push(5);

assert_eq!(buffer[0], 5);

// Second entry is now 42.
buffer.push(42);

// Because capacity is reached the next push will be the first item of the buffer.
buffer.push(1);
assert_eq!(buffer[0], 1);
```
