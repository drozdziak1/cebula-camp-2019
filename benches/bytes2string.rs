#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use cebula_camp::bytes2string;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bytes2string 1k-long", |b| {
        b.iter(|| bytes2string(black_box(vec![5u8; 1000].as_slice())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
