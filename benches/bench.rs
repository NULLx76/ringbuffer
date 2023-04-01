#![cfg(not(tarpaulin))]
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, RingBufferExt, RingBufferWrite};

fn benchmark_push<T: RingBufferExt<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    b.iter(|| {
        let mut rb = new();

        for i in 0..1_000_000 {
            rb.push(i)
        }

        rb
    })
}

fn benchmark_push_dequeue<T: RingBufferExt<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    b.iter(|| {
        let mut rb = new();

        for _i in 0..100_000 {
            rb.push(1);
            rb.push(2);

            assert_eq!(rb.dequeue(), Some(1));
            assert_eq!(rb.dequeue(), Some(2));

            rb.push(1);
            rb.push(2);

            assert_eq!(rb.dequeue(), Some(1));
            assert_eq!(rb.dequeue(), Some(2));

            rb.push(1);
            rb.push(2);

            assert_eq!(rb.get(-1), Some(&2));
            assert_eq!(rb.get(-2), Some(&1));
        }

        rb
    })
}

fn benchmark_various<T: RingBufferExt<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    b.iter(|| {
        let mut rb = new();

        for i in 0..100_000 {
            rb.push(i);
            black_box(rb.get(-1));
        }

        rb
    })
}

macro_rules! generate_benches {
    (called, $c: tt, $rb: tt, $ty: tt, $fn: tt, $bmfunc: tt, $($i:tt),*) => {
        $(
            $c.bench_function(&format!("{} {} 1M capacity {}", stringify!($rb), stringify!($bmfunc), stringify!($i)), |b| $bmfunc(b, || {
                $rb::<$ty>::$fn($i)
            }));
        )*
    };

    (typed, $c: tt, $rb: tt, $ty: tt, $fn: tt, $bmfunc: tt, $($i:tt),*) => {
        $(
            $c.bench_function(&format!("{} {} 1M capacity {}", stringify!($rb), stringify!($bmfunc) ,stringify!($i)), |b| $bmfunc(b, || {
                $rb::<$ty, $i>::$fn()
            }));
        )*
    };
}

fn benchmark_non_power_of_two<const L: usize>(b: &mut Bencher) {
    b.iter(|| {
        let mut rb = AllocRingBuffer::with_capacity_non_power_of_two(L);

        for i in 0..1_000_000 {
            rb.push(i)
        }

        rb
    })
}

fn benchmark_power_of_two<const L: usize>(b: &mut Bencher) {
    b.iter(|| {
        let mut rb = AllocRingBuffer::with_capacity(L);

        for i in 0..1_000_000 {
            rb.push(i)
        }

        rb
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    // TODO: Improve benchmarks
    // * What are representative operations
    // * Make sure it's accurate
    // * more general benchmarks but preferably less/quickjer

    generate_benches![
        called,
        c,
        AllocRingBuffer,
        i32,
        with_capacity,
        benchmark_push,
        16,
        1024,
        4096,
        8192
    ];
    generate_benches![
        typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_push,
        16,
        1024,
        4096,
        8192
    ];
    generate_benches![
        called,
        c,
        AllocRingBuffer,
        i32,
        with_capacity,
        benchmark_various,
        16,
        1024,
        4096,
        8192
    ];
    generate_benches![
        typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_various,
        16,
        1024,
        4096,
        8192
    ];
    generate_benches![
        called,
        c,
        AllocRingBuffer,
        i32,
        with_capacity,
        benchmark_push_dequeue,
        16,
        1024,
        4096,
        8192
    ];
    generate_benches![
        typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_push_dequeue,
        16,
        1024,
        4096,
        8192
    ];

    c.bench_function("power of two 16", benchmark_power_of_two::<16>);
    c.bench_function("non power of two 16", benchmark_non_power_of_two::<16>);

    c.bench_function("power of two 1024", benchmark_power_of_two::<1024>);
    c.bench_function("non power of two 1024", benchmark_non_power_of_two::<1024>);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
