#![feature(coverage_attribute)]
#![coverage(off)]
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, RingBuffer, SetLen};

fn benchmark_push<T: RingBuffer<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    b.iter(|| {
        let mut rb = new();

        for i in 0..1_000_000 {
            rb.enqueue(i);
            black_box(());
        }

        rb
    })
}

fn benchmark_push_dequeue<T: RingBuffer<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    b.iter(|| {
        let mut rb = new();

        for _i in 0..100_000 {
            let _ = rb.enqueue(1);
            black_box(());
            let _ = rb.enqueue(2);
            black_box(());

            assert_eq!(black_box(rb.dequeue()), Some(1));
            assert_eq!(black_box(rb.dequeue()), Some(2));

            let _ = rb.enqueue(1);
            black_box(());
            let _ = rb.enqueue(2);
            black_box(());

            assert_eq!(black_box(rb.dequeue()), Some(1));
            assert_eq!(black_box(rb.dequeue()), Some(2));

            let _ = rb.enqueue(1);
            black_box(());
            let _ = rb.enqueue(2);
            black_box(());

            assert_eq!(black_box(rb.get_signed(-1)), Some(&2));
            assert_eq!(black_box(rb.get_signed(-2)), Some(&1));
        }

        rb
    })
}

fn benchmark_various<T: RingBuffer<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    b.iter(|| {
        let mut rb = new();

        for i in 0..100_000 {
            rb.enqueue(i);
            black_box(());
            black_box(rb.back());
        }

        rb
    })
}

fn benchmark_skip<T: RingBuffer<i32>, F: Fn() -> T>(b: &mut Bencher, new: F) {
    let mut rb = new();
    rb.fill(9);
    b.iter(|| {
        for i in 0..rb.len() {
            assert_eq!(rb.iter().skip(i).next(), Some(&9));
        }
    })
}

fn benchmark_copy_to_slice_vs_extend<T: RingBuffer<i32>, F: Fn() -> T>(
    rb_size: usize,
    rb_type: &str,
    fn_name: &str,
    c: &mut Criterion,
    new: F,
) {
    let mut group = c.benchmark_group(format!("{fn_name}({rb_type}, {rb_size})"));
    let mut output = vec![0; rb_size];
    group.bench_function(format!("CopyTo({rb_type}; {rb_size})"), |b| {
        let mut rb = new();
        rb.fill(9);
        // making sure the read/write pointers wrap around
        for _ in 0..rb_size / 2 {
            let _ = rb.dequeue();
            let _ = rb.enqueue(9);
        }
        b.iter(|| {
            rb.copy_to_slice(0, &mut output);
            assert_eq!(output[output.len() / 2], 9);
            assert_eq!(output.len(), rb_size);
        })
    });
    let mut output: Vec<i32> = Vec::with_capacity(rb_size);
    group.bench_function(format!("ExtendVec({rb_type}; {rb_size})"), |b| {
        let mut rb = new();
        rb.fill(9);
        // making sure the read/write pointers wrap around
        for _ in 0..rb_size / 2 {
            let _ = rb.dequeue();
            let _ = rb.enqueue(9);
        }
        b.iter(|| {
            unsafe { output.set_len(0) };
            output.extend(rb.iter());
            assert_eq!(output[output.len() / 2], 9);
            assert_eq!(output.len(), rb_size);
        })
    });
    group.finish();
}

fn benchmark_copy_from_slice_vs_extend<T: RingBuffer<i32> + SetLen, F: Fn() -> T>(
    rb_size: usize,
    rb_type: &str,
    fn_name: &str,
    c: &mut Criterion,
    new: F,
) {
    let mut group = c.benchmark_group(format!("{fn_name}({rb_type}, {rb_size})"));
    let input = vec![9; rb_size];
    group.bench_function(format!("CopyFrom({rb_type}; {rb_size})"), |b| {
        let mut rb = new();
        rb.fill(0);
        // making sure the read/write pointers wrap around
        for _ in 0..rb_size / 2 {
            let _ = rb.dequeue();
            let _ = rb.enqueue(0);
        }
        for _ in 0..rb_size / 2 {}
        b.iter(|| {
            rb.copy_from_slice(0, &input);
            assert_eq!(rb[rb.len() / 2], 9);
            assert_eq!(rb.len(), rb_size);
        })
    });
    group.bench_function(format!("ExtendRb({rb_type}; {rb_size})"), |b| {
        let mut rb = new();
        // making sure the read/write pointers wrap around
        for _ in 0..rb_size / 2 {
            let _ = rb.dequeue();
            let _ = rb.enqueue(0);
        }
        b.iter(|| {
            unsafe { rb.set_len(0) };
            rb.extend(input.iter().copied());
            assert_eq!(rb[rb.len() / 2], 9);
            assert_eq!(rb.len(), rb_size);
        })
    });
    group.finish();
}

macro_rules! generate_benches {
    (called, $c: tt, $rb: tt, $ty: tt, $fn: tt, $bmfunc: tt, $($i:tt),*) => {
        $(
            $c.bench_function(&format!("{} {} 1M capacity {}", stringify!($rb), stringify!($bmfunc), stringify!($i)), |b| $bmfunc(b, || {
                $rb::<$ty>::$fn($i)
            }));
        )*
    };
    (non_power_two, $c: tt, $rb: tt, $ty: tt, $fn: tt, $bmfunc: tt, $($i:tt),*) => {
        $(
            $c.bench_function(&format!("{} {} 1M capacity not power of two {}", stringify!($rb), stringify!($bmfunc), stringify!($i)), |b| $bmfunc(b, || {
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

    (compare, $c: tt, $rb: tt, $ty: tt, $fn: tt, $bmfunc: tt, $($i:tt),*) => {
        $(
            $bmfunc($i, stringify!($rb), stringify!($bmfunc), $c, || {
                $rb::<$ty>::$fn($i)
            });
        )*
    };

    (compare_typed, $c: tt, $rb: tt, $ty: tt, $fn: tt, $bmfunc: tt, $($i:tt),*) => {
        $(
            $bmfunc($i, stringify!($rb), stringify!($bmfunc), $c, || {
                $rb::<$ty, $i>::$fn()
            });
        )*
    };
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
        non_power_two,
        c,
        AllocRingBuffer,
        i32,
        new,
        benchmark_various,
        16,
        17,
        1024,
        4096,
        8192,
        8195
    ];
    generate_benches![
        typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_skip,
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
        new,
        benchmark_skip,
        16,
        17,
        1024,
        4096,
        8192,
        8195
    ];
    generate_benches![
        compare,
        c,
        AllocRingBuffer,
        i32,
        new,
        benchmark_copy_to_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];
    generate_benches![
        compare_typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_copy_to_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];
    generate_benches![
        compare,
        c,
        AllocRingBuffer,
        i32,
        new,
        benchmark_copy_from_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];
    generate_benches![
        compare_typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_copy_from_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];

    generate_benches![
        compare,
        c,
        AllocRingBuffer,
        i32,
        new,
        benchmark_copy_to_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];
    generate_benches![
        compare_typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_copy_to_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];
    generate_benches![
        compare,
        c,
        AllocRingBuffer,
        i32,
        new,
        benchmark_copy_from_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];
    generate_benches![
        compare_typed,
        c,
        ConstGenericRingBuffer,
        i32,
        new,
        benchmark_copy_from_slice_vs_extend,
        16,
        1024,
        4096,
        8192,
        1_000_000,
        1_048_576
    ];
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
