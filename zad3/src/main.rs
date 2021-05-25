use rand::{distributions::Uniform, prelude::*};
use rand_distr::{Binomial, Normal};
use std::collections::{hash_map::RandomState, HashSet};
use std::iter::FromIterator;
use std::time::Instant;
use std::{collections::BTreeMap, time::Duration};
use zad3::{Bst, RbTree, Tree};

fn main() {
    let config_str = std::fs::read_to_string("config.ini").expect("error reading the file");
    let sizes: Vec<_> = config_str
        .lines()
        .next()
        .expect("no line with instance sizes found")
        .split_whitespace()
        .map(|s| s.parse().expect("error parsing int value"))
        .collect();

    let dist = Uniform::new_inclusive(0, u64::MAX);

    for n in sizes {
        let mut rng = SmallRng::seed_from_u64(0);
        let arr: Vec<u64> = (0..n).map(|_| dist.sample(&mut rng)).collect();
        let mut trees: [Box<dyn Tree<u64, u64>>; 2] =
            [Box::new(Bst::new()), Box::new(RbTree::new())];

        println!("N = {:?}", n);
        for tree in &mut trees {
            println!("{}", tree.name());
            // create
            {
                println!("");
                print!("creating: ");
                let create_time = bench(|| {
                    for &i in &arr {
                        tree.insert(i, i);
                    }
                });
                println!("{:?}", create_time);
            }

            // insert
            {
                let n_insrt = 100_000;
                print!("inserting {} elements:", n_insrt);
                let insert_time = bench(|| {
                    for _ in 0..n_insrt {
                        let num = dist.sample(&mut rng);
                        tree.insert(num, num);
                    }
                });
                println!("{:?}", insert_time);
            }

            // get
            {
                let n_searches: usize = 1_000;
                let index_dist = Uniform::from(0..arr.len());
                let keys: Vec<_> = (0..n_searches)
                    .map(|_| arr[index_dist.sample(&mut rng)])
                    .collect();
                print!("searching {} elements:", n_searches);
                let search_time = bench(|| {
                    for i in 0..n_searches {
                        let res = tree.get(&keys[i]);
                        assert_eq!(res, Some(&keys[i]));
                    }
                });
                println!("{:?}", search_time);
            }

            // remove
            {
                let n_deletions: usize = 1_000;
                let index_dist = Uniform::from(0..arr.len());
                let keys = (0..arr.len() - 1)
                    .collect::<Vec<_>>()
                    .partial_shuffle(&mut rng, n_deletions)
                    .0
                    .to_vec()
                    .into_iter()
                    .map(|i| arr[i])
                    .collect::<Vec<_>>();

                print!("removing {} elements:", n_deletions);
                let remove_time = bench(|| {
                    for i in 0..n_deletions {
                        let res = tree.remove(&keys[i]);
                    }
                });
                println!("{:?}", remove_time);
            }

            println!("");
        }
    }
}

fn bench(mut f: impl FnMut()) -> Duration {
    let start = Instant::now();
    f();
    Instant::elapsed(&start)
}
