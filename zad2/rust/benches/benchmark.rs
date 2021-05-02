use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Uniform, prelude::*};
use zad2_sorting::{quicksort, radixsort};

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");

    let mut rng = SmallRng::seed_from_u64(0);
    let dist = Uniform::from(0..u32::MAX);
    let sizes = [10_000, 20_000, 30_000, 40_000, 50_000];

    for &size in &sizes {
        let arr: Vec<u32> = (0..size).map(|_| dist.sample(&mut rng)).collect();

        group.bench_with_input(BenchmarkId::new("quicksort", size), &size, |b, _size| {
            b.iter_batched_ref(
                || arr.clone(),
                |mut data| quicksort(&mut data),
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("radixsort", size), &size, |b, _size| {
            b.iter_batched_ref(
                || arr.clone(),
                |mut data| radixsort(&mut data),
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("stdsort", size), &size, |b, _size| {
            b.iter_batched_ref(
                || arr.clone(),
                |data| data.sort(),
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
