# Ringbuffer
![Github Workflows](https://img.shields.io/github/actions/workflow/status/NULLx76/ringbuffer/rust.yml?style=for-the-badge)
[![Docs.rs](https://img.shields.io/badge/docs.rs-ringbuffer-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/ringbuffer)
[![Crates.io](https://img.shields.io/crates/v/ringbuffer?logo=rust&style=for-the-badge)](https://crates.io/crates/ringbuffer)

The ringbuffer crate provides safe fixed size circular buffers (ringbuffers) in rust.

Implementations for three kinds of ringbuffers, with a mostly similar API are provided:

| type                           | description                                                                                                                                                            |
|--------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`AllocRingBuffer`][1]         | Ringbuffer allocated on the heap at runtime. This ringbuffer is still fixed size. This requires the alloc feature.                                                     |
| [`GrowableAllocRingBuffer`][2] | Ringbuffer allocated on the heap at runtime. This ringbuffer can grow in size, and is implemented as an `alloc::VecDeque` internally. This requires the alloc feature. |
| [`ConstGenericRingBuffer`][3]  | Ringbuffer which uses const generics to allocate on the stack.                                                                                                         |

All of these ringbuffers also implement the [RingBuffer][4] trait for their shared API surface.

[1]: https://docs.rs/ringbuffer/latest/ringbuffer/struct.AllocRingBuffer.html
[2]: https://docs.rs/ringbuffer/latest/ringbuffer/struct.GrowableAllocRingBuffer.html
[3]: https://docs.rs/ringbuffer/latest/ringbuffer/struct.ConstGenericRingBuffer.html
[4]: https://docs.rs/ringbuffer/latest/ringbuffer/trait.RingBuffer.html

MSRV: Rust 1.59

# Usage

```rust
use ringbuffer::{AllocRingBuffer, RingBuffer};

fn main() {
    let mut buffer = AllocRingBuffer::with_capacity(2);

    // First entry of the buffer is now 5.
    buffer.push(5);

    // The last item we pushed is 5
    assert_eq!(buffer.get(-1), Some(&5));

    // Second entry is now 42.
    buffer.push(42);
    assert_eq!(buffer.peek(), Some(&5));
    assert!(buffer.is_full());

    // Because capacity is reached the next push will be the first item of the buffer.
    buffer.push(1);
    assert_eq!(buffer.to_vec(), vec![42, 1]);
}

```

# Benchmarks

## Single operations

For all benchmarks here, we take `40 000` measurements over 60 seconds, usually resulting in a couple of billion iterations executed. The benchmarks were executed on a dedicated core (no scheduling overhead).
The accuracy of this is good enough that two runs in a row show no (significant) difference in performance.
Hardware: AMD Ryzen 9 5900HX with the following caches:
```
L1d:                   256 KiB (8 instances)
L1i:                   256 KiB (8 instances)
L2:                    4 MiB (8 instances)
L3:                    16 MiB (1 instance)
```

A full ringbuffer means it was first filled *to capacity* for push benchmarks, and for dequeue benchmarks it means that there is indeed an item in the ringbuffer to dequeue, as opposed to the empty versions where the dequeue operation returns `None`

| benchmark           | capacity | variant      | alloc ringbuffer | const generic ringbuffer | growable ringbuffer | 
|---------------------|----------|--------------|------------------|--------------------------|---------------------|
| push single item    | 16       | buffer full  | 2.50ns           | 2.14ns                   | 45.2ns              | 
|                     | 10       | buffer full  | 2.22ns           | 1.84ns                   | 45.5ns              |
|                     | 16       | buffer empty | 1.94ns           | 1.56ns                   | 1.98ns              |
|                     | 10       | buffer empty | 2.22ns           | 1.41ns                   | 1.72ns              |
| dequeue single item | 16       | buffer full  | 1.97ns           | 1.69ns                   | 1.72ns              |
|                     | 10       | buffer full  | 1.72ns           | 1.51ns                   | 1.84ns              |
|                     | 16       | buffer empty | 826ps            | 1.48ns                   | 746ps               |
|                     | 10       | buffer empty | 820ps            | 949ps                    | 758ps               |

## Batches

For these benchmarks, the same hardware was used, but only `20 000` samples were taken since each instance takes longer.
Draining the buffer means removing capacity items from a ringbuffer filled to capacity.

| benchmark                     | capacity | alloc ringbuffer | const generic ringbuffer | growable ringbuffer |
|-------------------------------|----------|------------------|--------------------------|---------------------|
| fill buffer (1x capacity)     | 1024     | 1.11µs           | 1.01µs                   | 1.18µs              | 
|                               | 1000     | 1.90µs           | 1.27µs                   | 1.15µs              | 
| overfill buffer (2x capacity) | 1024     | 2.43µs           | 2.30µs                   | 2.58µs              |
|                               | 1000     | 3.81µs           | 2.53µs                   | 2.43µs              |
| drain buffer                  | 1024     | 84ns             | 610ns                    | 1.45µs              |
|                               | 1000     | 1.88µs           | 93ns                     | 1.40µs              |


# Features

| name  | default | description                                                                                                  |
|-------|---------|--------------------------------------------------------------------------------------------------------------|
| alloc | ✓       | Disable this feature to remove the dependency on alloc. Disabling this feature  makes `ringbuffer` `no_std`. |

# License

Licensed under MIT License
