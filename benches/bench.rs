#![cfg(not(tarpaulin))]
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, ModFreeRingBuffer, RingBufferExt, RingBufferWrite};
use core::num::NonZeroUsize;

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

fn benchmark_mod_free<const L: usize>(b: &mut Bencher) {
    b.iter(|| {
        let mut rb = ModFreeRingBuffer::new(NonZeroUsize::new(L).unwrap());

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

    let [nz16, nz1024, nz4096, nz8192] =
        [16, 1024, 4096, 8192].map(|n| NonZeroUsize::new(n).unwrap());

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
        ModFreeRingBuffer,
        i32,
        new,
        benchmark_push,
        nz16,
        nz1024,
        nz4096,
        nz8192
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
        ModFreeRingBuffer,
        i32,
        new,
        benchmark_various,
        nz16,
        nz1024,
        nz4096,
        nz8192
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
    generate_benches![
        called,
        c,
        ModFreeRingBuffer,
        i32,
        new,
        benchmark_push_dequeue,
        nz16,
        nz1024,
        nz4096,
        nz8192
    ];

    c.bench_function("manual: power of two 16", benchmark_power_of_two::<16>);
    c.bench_function("manual: non power of two 16", benchmark_non_power_of_two::<16>);
    c.bench_function("manual: mod free 16", benchmark_mod_free::<16>);

    c.bench_function("manual: power of two 1024", benchmark_power_of_two::<1024>);
    c.bench_function("manual: non power of two 1024", benchmark_non_power_of_two::<1024>);
    c.bench_function("manual: mod free 1024", benchmark_mod_free::<1024>);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
