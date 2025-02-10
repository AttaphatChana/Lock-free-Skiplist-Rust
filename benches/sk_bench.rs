use std::iter;
use criterion::{criterion_group, criterion_main, black_box, Criterion, Throughput, BenchmarkId};
use rand::Rng;
use SkipList::{sk_test, something,import_sk};
use crossbeam_epoch::{Guard,self as epoch};
use skiplist::OrderedSkipList;

// fn import_bench(c: &mut Criterion) {
//     let mut group = c.benchmark_group("import_sk");
//
//     for size in [1, 5,10, 50,100, 500,1000, 5000,10000, 50000,100000, 500000,1000000] {
//         //group.throughput(Throughput::Bytes(*size as u64));
//         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| unsafe {
//             b.iter(|| {
//                 import_sk(black_box(size));
//             })
//         });
//     }
//
// }

fn sk_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sk_insert_bench");
    // c.bench_function("sk_bench", |b| {
    //     b.iter(|| {
    //         let mut rng = rand::rng();
    //         let n = 1000000;
    //         let sequence: Vec<i32> = (0..n).map(|_| rng.random_range(-n..n)).collect();
    //             sk_test(black_box(sequence));
    //         })
    //     });
    for size in [1000, 5000,10000, 50000,100000, 500000,1000000] {
        //group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("sk_insert",size), &size, |b, &size| unsafe {
            b.iter(|| {
                sk_test(black_box(size));
            })
        });
        group.bench_with_input(BenchmarkId::new("built-in_sk_insert",size), &size, |b, &size| unsafe {
            b.iter(|| {
                import_sk(black_box(size));
            })
        });

    }
    group.finish();
}


fn search(c: &mut Criterion) {
    // static KB: usize = 1024;

    let mut group = c.benchmark_group("search");
    let n = 1000000;
    let mut sk = something::SkipList::new();
    let mut bt = OrderedSkipList::new();
    let guard = epoch::pin();
    (0.. n).for_each(|i| unsafe {
        sk.insert(&guard,i);
    });
    (0.. n).for_each(|i| unsafe {
        bt.insert(i);
    });
    for size in [1000, 5000,10000, 50000,100000, 500000,1000000].iter() {
        //group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("sk_search ", size), &size, |b, &size| unsafe {
            b.iter(|| sk.search_n(black_box(&guard),black_box(*size)));
        });
        group.bench_with_input(BenchmarkId::new("built-in_sk_search ", size), &size, |b, &size| unsafe {
            b.iter(|| bt.contains(black_box(size)));
        });

    }
    group.finish();
}

criterion_group!(
    benches,
    search,
    sk_bench,
);
criterion_main!(benches);