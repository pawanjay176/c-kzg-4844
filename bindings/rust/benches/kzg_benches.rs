
use c_kzg::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("verify_kzg_proof", |b| {
        b.iter(|| {
            verify_kzg_proof_wrapper()
        })
    });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
