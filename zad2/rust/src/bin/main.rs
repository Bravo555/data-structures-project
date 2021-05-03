use std::time::Instant;

use rand::{distributions::Uniform, prelude::*};
use zad2_sorting::{quicksort, radixsort};

fn main() {
    const N: usize = 1_000_000;
    println!("measuring: {} elements", N);

    let mut rng = SmallRng::seed_from_u64(0);
    let dist = Uniform::from(0..1000);
    let mut arr: Vec<u32> = (0..N).map(|_| dist.sample(&mut rng)).collect();

    let mut quicksort_arr = arr.clone();
    let mut radixsort_arr = arr.clone();

    let start = Instant::now();
    quicksort(&mut quicksort_arr);
    let quicksort_time = Instant::elapsed(&start);

    let start = Instant::now();
    radixsort(&mut radixsort_arr);
    let radixsort_time = Instant::elapsed(&start);

    let start = Instant::now();
    arr.sort();
    let stdsort_time = Instant::elapsed(&start);

    assert_eq!(quicksort_arr, arr);
    assert_eq!(radixsort_arr, arr);

    println!("Quicksort took: {:?}", quicksort_time);
    println!("Radixsort took: {:?}", radixsort_time);
    println!("Stdsort took: {:?}", stdsort_time);
}
