use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use lds_rs::lds::PRIME_TABLE;
use sphere_n_rs::{CylindGen, CylindN, SphereN};

fn bench_sphere3(c: &mut Criterion) {
    c.bench_function("sphere3_pop", |b| {
        let mut sgen = SphereN::new(3, &PRIME_TABLE);
        sgen.reseed(0);
        b.iter(|| {
            black_box(sgen.pop_vec());
        });
    });
}

fn bench_sphere5(c: &mut Criterion) {
    c.bench_function("sphere5_pop", |b| {
        let mut sgen = SphereN::new(5, &PRIME_TABLE);
        sgen.reseed(0);
        b.iter(|| {
            black_box(sgen.pop_vec());
        });
    });
}

fn bench_cylind5(c: &mut Criterion) {
    c.bench_function("cylind5_pop", |b| {
        let mut cgen = CylindN::new(5, &PRIME_TABLE);
        cgen.reseed(0);
        b.iter(|| {
            black_box(cgen.pop_vec());
        });
    });
}

fn bench_sphere10(c: &mut Criterion) {
    c.bench_function("sphere10_pop", |b| {
        let mut sgen = SphereN::new(10, &PRIME_TABLE);
        sgen.reseed(0);
        b.iter(|| {
            black_box(sgen.pop_vec());
        });
    });
}

criterion_group!(
    benches,
    bench_sphere3,
    bench_sphere5,
    bench_cylind5,
    bench_sphere10
);
criterion_main!(benches);
