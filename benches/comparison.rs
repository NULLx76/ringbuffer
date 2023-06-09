#![cfg(not(tarpaulin))]

use std::collections::{LinkedList, VecDeque};
use std::sync::mpsc::channel;
use criterion::{black_box, criterion_group, Bencher, Criterion};
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, RingBuffer};

const ITER: usize = 1024 * 16;
const CAP: usize = 1024;

fn std_chan(b: &mut Bencher) {
    let (tx, rx) = channel();

    b.iter(|| {
        for i in 0..ITER {
            let _ = tx.send(i);
            black_box(());
        }

        for i in 0..ITER {
            let res = rx.recv();
            let _ = black_box(res);
        }
    });
}

fn vec(b: &mut Bencher) {
    let mut vd = Vec::with_capacity(CAP);

    b.iter(|| {
        for i in 0..ITER {
            let _ = vd.push(i);
            black_box(());
        }

        for i in 0..ITER {
            let res = vd.remove(0);
            let _ = black_box(res);
        }
    });
}

fn vecdeque(b: &mut Bencher) {
    let mut vd = VecDeque::with_capacity(CAP);

    b.iter(|| {
        for i in 0..ITER {
            let _ = vd.push_back(i);
            black_box(());
        }
        for i in 0..ITER {
            let res = vd.pop_front();
            let _ = black_box(res);
        }
    });
}

fn linked_list(b: &mut Bencher) {
    let mut ll = LinkedList::new();

    b.iter(|| {
        for i in 0..ITER {
            let _ = ll.push_back(i);
            black_box(());
        }

        for i in 0..ITER {
            let res = ll.pop_front();
            let _ = black_box(res);
        }
    });
}

fn cg_rb(b: &mut Bencher) {
    let mut rb = ConstGenericRingBuffer::<_, CAP>::new();

    b.iter(|| {
        for i in 0..ITER {
            let _ = rb.push(i);
            black_box(());
        }
        for i in 0..ITER {
            let res = rb.dequeue();
            let _ = black_box(res);
        }
    });
}

fn heapless_deque(b: &mut Bencher) {
    let mut rb = heapless::Deque::<_, CAP>::new();

    b.iter(|| {
        for i in 0..ITER {
            let _ = rb.push_back(i);
            black_box(());
        }
        for i in 0..ITER {
            let res = rb.pop_front();
            let _ = black_box(res);
        }
    });
}

fn al_rb(b: &mut Bencher) {
    let mut rb = AllocRingBuffer::with_capacity_non_power_of_two(CAP);

    b.iter(|| {
        for i in 0..ITER {
            let _ = rb.push(i);
            black_box(());
        }
        for i in 0..ITER {
            let res = rb.dequeue();
            let _ = black_box(res);
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("comparison std channel", std_chan);
    c.bench_function("comparison std vec", vec);
    c.bench_function("comparison std linked list", linked_list);
    c.bench_function("comparison std vecdeque (growable ringbuffer)", vecdeque);
    c.bench_function("comparison const generic ringbuffer", cg_rb);
    c.bench_function("comparison alloc ringbuffer", al_rb);
    c.bench_function("comparison heapless deque", heapless_deque);
}

criterion_group!(comparison_benches, criterion_benchmark);
