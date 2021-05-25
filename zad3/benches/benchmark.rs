use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Uniform, prelude::*};
use zad3::{Bst, RbTree, Tree};

pub fn benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(0);
    let dist = Uniform::from(0..u64::MAX);
    let sizes = [1_000, 2_000, 3_000, 4_000, 5_000];

    let mut create_g = c.benchmark_group("create");
    for &size in &sizes {
        let arr: Vec<_> = (0..size).map(|_| dist.sample(&mut rng)).collect();

        create_g.bench_with_input(BenchmarkId::new("BST", size), &size, |b, _size| {
            b.iter_batched_ref(
                || Bst::new(),
                |tree| {
                    for i in &arr {
                        tree.insert(*i, *i);
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });

        create_g.bench_with_input(BenchmarkId::new("RbTree", size), &size, |b, _size| {
            b.iter_batched_ref(
                || RbTree::new(),
                |tree| {
                    for i in &arr {
                        tree.insert(*i, *i);
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    create_g.finish();

    let mut insert_g = c.benchmark_group("insert");
    for &size in &sizes {
        let arr: Vec<_> = (0..size).map(|_| dist.sample(&mut rng)).collect();
        let mut bst = Bst::new();
        for i in &arr {
            bst.insert(*i, *i);
        }
        let mut rbtree = RbTree::new();
        for i in &arr {
            rbtree.insert(*i, *i);
        }

        insert_g.bench_with_input(BenchmarkId::new("BST", size), &size, |b, _size| {
            b.iter_batched_ref(
                || bst.clone(),
                |tree| {
                    let val = dist.sample(&mut rng);
                    tree.insert(val, val);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        insert_g.bench_with_input(BenchmarkId::new("RbTree", size), &size, |b, _size| {
            b.iter_batched_ref(
                || rbtree.clone(),
                |tree| {
                    let val = dist.sample(&mut rng);
                    tree.insert(val, val);
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    insert_g.finish();

    let mut get_g = c.benchmark_group("find");
    for &size in &sizes {
        let arr: Vec<_> = (0..size).map(|_| dist.sample(&mut rng)).collect();
        let mut bst = Bst::new();
        for i in &arr {
            bst.insert(*i, *i);
        }
        let mut rbtree = RbTree::new();
        for i in &arr {
            rbtree.insert(*i, *i);
        }
        let index_dist = Uniform::from(0..arr.len());

        get_g.bench_with_input(BenchmarkId::new("BST", size), &size, |b, _size| {
            b.iter_batched_ref(
                || bst.clone(),
                |tree| {
                    let val = arr[index_dist.sample(&mut rng)];
                    tree.get(&val);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        get_g.bench_with_input(BenchmarkId::new("RbTree", size), &size, |b, _size| {
            b.iter_batched_ref(
                || bst.clone(),
                |tree| {
                    let val = arr[index_dist.sample(&mut rng)];
                    tree.get(&val);
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    get_g.finish();

    let mut remove_g = c.benchmark_group("remove");
    for &size in &sizes {
        let arr: Vec<_> = (0..size).map(|_| dist.sample(&mut rng)).collect();
        let mut bst = Bst::new();
        for i in &arr {
            bst.insert(*i, *i);
        }
        let mut rbtree = RbTree::new();
        for i in &arr {
            rbtree.insert(*i, *i);
        }
        let index_dist = Uniform::from(0..arr.len());

        remove_g.bench_with_input(BenchmarkId::new("BST", size), &size, |b, _size| {
            b.iter_batched_ref(
                || bst.clone(),
                |tree| {
                    let val = arr[index_dist.sample(&mut rng)];
                    tree.remove(&val);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        remove_g.bench_with_input(BenchmarkId::new("RbTree", size), &size, |b, _size| {
            b.iter_batched_ref(
                || rbtree.clone(),
                |tree| {
                    let val = arr[index_dist.sample(&mut rng)];
                    tree.remove(&val);
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    remove_g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
