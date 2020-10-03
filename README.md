# ringbuffer
[![Drone (self-hosted)](https://img.shields.io/drone/build/0x76/ringbuffer?logo=drone&server=https%3A%2F%2Fdrone.xirion.net&style=for-the-badge)](https://drone.xirion.net/0x76/ringbuffer)
[![MIT License](https://img.shields.io/badge/license-LGPL3-blue?style=for-the-badge)](./LICENSE)

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
