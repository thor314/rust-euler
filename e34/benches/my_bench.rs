use criterion::{black_box, criterion_group, criterion_main, Criterion};
use e34::*;

pub fn og_curious_og_factorial(c: &mut Criterion) {
    c.bench_function("blah", |b| b.iter(|| e()));
}

pub fn og_curious_alice_factorial(c: &mut Criterion) {
    c.bench_function("blah1", |b| b.iter(|| e1()));
}

pub fn alice_curious_og_factorial(c: &mut Criterion) {
    c.bench_function("blah2", |b| b.iter(|| e2()));
}

pub fn alice_curious_alice_factorial(c: &mut Criterion) {
    c.bench_function("blah2", |b| b.iter(|| e3()));
}

criterion_group!(benches, og_curious_og_factorial);
//criterion_group!(benches1, og_curious_alice_factorial);
//criterion_group!(benches2, alice_curious_og_factorial);
//criterion_group!(benches3, alice_curious_alice_factorial);
criterion_main!(benches); //, benches1, benches2, benches3);
