# ringbuffer
[![Drone (self-hosted)](https://img.shields.io/drone/build/0x76/ringbuffer?logo=drone&server=https%3A%2F%2Fdrone.xirion.net&style=for-the-badge)](https://drone.xirion.net/0x76/ringbuffer)
[![Codecov](https://img.shields.io/codecov/c/gh/NULLx76/ringbuffer?logo=codecov&style=for-the-badge)](https://codecov.io/gh/NULLx76/ringbuffer)
[![Docs.rs](https://img.shields.io/badge/docs.rs-ringbuffer-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/ringbuffer)
[![Crates.io](https://img.shields.io/crates/v/ringbuffer?logo=rust&style=for-the-badge)](https://crates.io/crates/ringbuffer)
[![LPGL License](https://img.shields.io/badge/license-LGPL3-blue?style=for-the-badge)](./LICENSE)

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
