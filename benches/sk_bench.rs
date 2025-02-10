use std::iter;
use criterion::{criterion_group, criterion_main, black_box, Criterion};
use rand::Rng;
use SkipList::sk_test;
fn sk_bench(c: &mut Criterion) {
    c.bench_function("sk_bench", |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let n = 1000000;
            let sequence: Vec<i32> = (0..n).map(|_| rng.random_range(-n..n)).collect();
                sk_test(black_box(sequence));
            })
        });
}

criterion_group!(
    benches,
    sk_bench,
);
criterion_main!(benches);