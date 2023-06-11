#![cfg(not(tarpaulin))]

use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion, BatchSize, SamplingMode};
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, GrowableAllocRingBuffer, RingBuffer};


fn criterion_benchmark(c: &mut Criterion) {
    let mut single_ops = c.benchmark_group("single-ops");
    single_ops.nresamples(20000);
    single_ops.measurement_time(Duration::from_secs(60));
    single_ops.sampling_mode(SamplingMode::Linear);
    single_ops.warm_up_time(Duration::from_secs(5));
    single_ops.significance_level(0.05).sample_size(40_000);
    const SINGLE_BATCH_SIZE: BatchSize = BatchSize::NumBatches(1);

    macro_rules! push_empty {
        ($constructor: expr, $name: literal) => {
            {
                #[inline]
                fn push_empty(b: &mut Bencher) {
                    b.iter_batched_ref(|| {
                        $constructor
                    }, |rb| {
                        black_box(rb.push(black_box(1)));
                    }, SINGLE_BATCH_SIZE)
                }
                single_ops.bench_function(concat!("push empty: ", $name), push_empty);
            }
        };
    }

    macro_rules! push_full {
        ($constructor: expr, $name: literal) => {
            {
                #[inline]
                fn push_full(b: &mut Bencher) {
                    b.iter_batched_ref(|| {
                        let mut a = $constructor;

                        for i in 0..a.capacity() {
                            a.push(i);
                        }

                        a
                    }, |rb| {
                        black_box(rb.push(black_box(1)));
                    }, SINGLE_BATCH_SIZE)
                }
                single_ops.bench_function(concat!("push full: ", $name), push_full);
            }
        };
    }

    macro_rules! dequeue_with_item {
        ($constructor: expr, $name: literal) => {
            {
                #[inline]
                fn push_empty(b: &mut Bencher) {
                    b.iter_batched_ref(|| {
                        let mut a = $constructor;
                        a.push(1);
                        a
                    }, |rb| {
                        black_box(rb.dequeue());
                    }, SINGLE_BATCH_SIZE)
                }
                single_ops.bench_function(concat!("dequeue with item: ", $name), push_empty);
            }
        };
    }

    macro_rules! dequeue_empty {
        ($constructor: expr, $name: literal) => {
            {
                #[inline]
                fn push_empty(b: &mut Bencher) {
                    b.iter_batched_ref(|| {
                        $constructor
                    }, |rb| {
                        let _: Option<i32> = black_box(rb.dequeue());
                    }, SINGLE_BATCH_SIZE)
                }
                single_ops.bench_function(concat!("dequeue empty: ", $name), push_empty);
            }
        };
    }

    {
        push_empty!(AllocRingBuffer::with_capacity_non_power_of_two(10), "[alloc] non-power-of-two");
        push_empty!(ConstGenericRingBuffer::<_, 10>::new(), "[const-generic] non-power-of-two");
        push_empty!(GrowableAllocRingBuffer::with_capacity(10), "[growable] non-power-of-two");

        push_empty!(AllocRingBuffer::new(16), "[alloc] power-of-two");
        push_empty!(ConstGenericRingBuffer::<_, 16>::new(), "[const-generic] power-of-two");
        push_empty!(GrowableAllocRingBuffer::with_capacity(16), "[growable] power-of-two");
    }

    {
        push_full!(AllocRingBuffer::with_capacity_non_power_of_two(10), "[alloc] non-power-of-two");
        push_full!(ConstGenericRingBuffer::<_, 10>::new(), "[const-generic] non-power-of-two");
        push_full!(GrowableAllocRingBuffer::with_capacity(10), "[growable] non-power-of-two");

        push_full!(AllocRingBuffer::new(16), "[alloc] power-of-two");
        push_full!(ConstGenericRingBuffer::<_, 16>::new(), "[const-generic] power-of-two");
        push_full!(GrowableAllocRingBuffer::with_capacity(16), "[growable] power-of-two");
    }

    {
        dequeue_with_item!(AllocRingBuffer::with_capacity_non_power_of_two(10), "[alloc] non-power-of-two");
        dequeue_with_item!(ConstGenericRingBuffer::<_, 10>::new(), "[const-generic] non-power-of-two");
        dequeue_with_item!(GrowableAllocRingBuffer::with_capacity(10), "[growable] non-power-of-two");

        dequeue_with_item!(AllocRingBuffer::new(16), "[alloc] power-of-two");
        dequeue_with_item!(ConstGenericRingBuffer::<_, 16>::new(), "[const-generic] power-of-two");
        dequeue_with_item!(GrowableAllocRingBuffer::with_capacity(16), "[growable] power-of-two");
    }

    {
        dequeue_empty!(AllocRingBuffer::with_capacity_non_power_of_two(10), "[alloc] non-power-of-two");
        dequeue_empty!(ConstGenericRingBuffer::<_, 10>::new(), "[const-generic] non-power-of-two");
        dequeue_empty!(GrowableAllocRingBuffer::with_capacity(10), "[growable] non-power-of-two");

        dequeue_empty!(AllocRingBuffer::new(16), "[alloc] power-of-two");
        dequeue_empty!(ConstGenericRingBuffer::<_, 16>::new(), "[const-generic] power-of-two");
        dequeue_empty!(GrowableAllocRingBuffer::with_capacity(16), "[growable] power-of-two");
    }
    single_ops.finish();

    let mut batch = c.benchmark_group("batch");
    batch.nresamples(20000);
    batch.measurement_time(Duration::from_secs(60));
    batch.sampling_mode(SamplingMode::Linear);
    batch.warm_up_time(Duration::from_secs(5));
    batch.significance_level(0.05).sample_size(1000);

    const BATCH_BATCH_SIZE: BatchSize = BatchSize::LargeInput;

    macro_rules! fill {
        ($constructor: expr, $name: literal) => {
            {
                #[inline]
                fn push_empty(b: &mut Bencher) {
                    b.iter_batched_ref(|| {
                        $constructor
                    }, |rb| {
                        for i in 0..rb.capacity() {
                            rb.push(black_box(i));
                        }
                    }, BATCH_BATCH_SIZE)
                }
                batch.bench_function(concat!("fill: ", $name), push_empty);
            }
        };
    }

    macro_rules! over_fill {
        ($constructor: expr, $name: literal) => {
            {
                #[inline]
                fn push_empty(b: &mut Bencher) {
                    b.iter_batched_ref(|| {
                        $constructor
                    }, |rb| {
                        for i in 0..(rb.capacity() * 2) {
                            rb.push(black_box(i));
                        }
                    }, BATCH_BATCH_SIZE)
                }
                batch.bench_function(concat!("over fill: ", $name), push_empty);
            }
        };
    }

    macro_rules! drain {
        ($constructor: expr, $name: literal) => {
            {
                #[inline]
                fn push_empty(b: &mut Bencher) {
                    b.iter_batched_ref(|| {
                        let mut rb = $constructor;

                        for i in 0..rb.capacity() {
                            rb.push(i);
                        }

                        rb
                    }, |rb| {
                        while let Some(i) = rb.dequeue() {
                            black_box(i);
                        }
                    }, BATCH_BATCH_SIZE)
                }
                batch.bench_function(concat!("drain: ", $name), push_empty);
            }
        };
    }


    {
        fill!(AllocRingBuffer::with_capacity_non_power_of_two(1000), "[alloc] non-power-of-two");
        fill!(ConstGenericRingBuffer::<_, 1000>::new(), "[const-generic] non-power-of-two");
        fill!(GrowableAllocRingBuffer::with_capacity(1000), "[growable] non-power-of-two");

        fill!(AllocRingBuffer::new(1024), "[alloc] power-of-two");
        fill!(ConstGenericRingBuffer::<_, 1024>::new(), "[const-generic] power-of-two");
        fill!(GrowableAllocRingBuffer::with_capacity(1023), "[growable] power-of-two");
    }

    {
        over_fill!(AllocRingBuffer::with_capacity_non_power_of_two(1000), "[alloc] non-power-of-two");
        over_fill!(ConstGenericRingBuffer::<_, 1000>::new(), "[const-generic] non-power-of-two");
        over_fill!(GrowableAllocRingBuffer::with_capacity(1000), "[growable] non-power-of-two");

        over_fill!(AllocRingBuffer::new(1024), "[alloc] power-of-two");
        over_fill!(ConstGenericRingBuffer::<_, 1024>::new(), "[const-generic] power-of-two");
        over_fill!(GrowableAllocRingBuffer::with_capacity(1023), "[growable] power-of-two");
    }

    {
        drain!(AllocRingBuffer::with_capacity_non_power_of_two(1000), "[alloc] non-power-of-two");
        drain!(ConstGenericRingBuffer::<_, 1000>::new(), "[const-generic] non-power-of-two");
        drain!(GrowableAllocRingBuffer::with_capacity(1000), "[growable] non-power-of-two");

        drain!(AllocRingBuffer::new(1024), "[alloc] power-of-two");
        drain!(ConstGenericRingBuffer::<_, 1024>::new(), "[const-generic] power-of-two");
        drain!(GrowableAllocRingBuffer::with_capacity(1023), "[growable] power-of-two");
    }

    batch.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
