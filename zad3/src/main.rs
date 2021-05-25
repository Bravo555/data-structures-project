use rand::{distributions::Uniform, prelude::*};
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
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

    let mut results_file = File::create("results.csv").unwrap();
    writeln!(results_file, "type,operation,instance,time").unwrap();

    for n in sizes {
        let mut trees: [Box<dyn Tree<u64, u64>>; 2] =
            [Box::new(Bst::new()), Box::new(RbTree::new())];

        println!("N = {:?}", n);
        for tree in &mut trees {
            let mut rng = SmallRng::seed_from_u64(0);
            let arr: Vec<u64> = (0..n).map(|_| dist.sample(&mut rng)).collect();
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
                writeln!(
                    results_file,
                    "{},{},{},{}",
                    tree.name(),
                    "create",
                    n,
                    &create_time.as_micros()
                )
                .unwrap();
            }

            // insert
            {
                let n_insrt = 10_000;
                print!("inserting {} elements:", n_insrt);
                let insert_time = bench(|| {
                    for _ in 0..n_insrt {
                        let num = dist.sample(&mut rng);
                        tree.insert(num, num);
                    }
                });
                println!("{:?}", insert_time);
                writeln!(
                    results_file,
                    "{},{},{},{}",
                    tree.name(),
                    "insert",
                    n,
                    &insert_time.as_micros()
                )
                .unwrap();
            }

            // get
            {
                let n_searches: usize = 10_000;
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
                writeln!(
                    results_file,
                    "{},{},{},{}",
                    tree.name(),
                    "search",
                    n,
                    &search_time.as_micros()
                )
                .unwrap();
            }

            // remove
            {
                let n_deletions: usize = 10_000;
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
                writeln!(
                    results_file,
                    "{},{},{},{}",
                    tree.name(),
                    "remove",
                    n,
                    &remove_time.as_micros()
                )
                .unwrap();
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
