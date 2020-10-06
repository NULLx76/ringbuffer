#[macro_use]
extern crate criterion;

use criterion::{black_box, Bencher, Criterion};
use ringbuffer::typenum::*;
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, GenericRingBuffer, RingBuffer};

fn benchmark_push<T: RingBuffer<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    b.iter(|| {
        let mut rb = new();

        for i in 0..1_000_000 {
            rb.push(i)
        }

        rb
    })
}

fn benchmark_push_dequeue<T: RingBuffer<i32>, F: Fn() -> T>(b: &mut Bencher, ew: F) {
    b.iter(|| {
        let mut rb = new();

        for i in 0..100_000 {
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

fn benchmark_various<T: RingBuffer<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
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

fn criterion_benchmark(c: &mut Criterion) {
    c.with_plots();

    generat_benches![
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
        typed,
        c,
        GenericRingBuffer,
        i32,
        new,
        benchmark_push,
        U16,
        U1024,
        U4096,
        U8192
    ];

    generat_benches![
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
        typed,
        c,
        GenericRingBuffer,
        i32,
        new,
        benchmark_various,
        U16,
        U1024,
        U4096,
        U8192
    ];

    generate_benche![
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
        typed,
        c,
        GenericRingBuffer,
        i32,
        new,
        benchmark_push_dequeue,
        U16,
        U1024,
        U4096,
        U8192
    ];
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
