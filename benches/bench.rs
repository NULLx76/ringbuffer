#![cfg(not(tarpaulin))]
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, RingBufferExt, RingBufferWrite};

pub fn bench_push() {

}

fn criterion_benchmark(c: &mut Criterion) {

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
