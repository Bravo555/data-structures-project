use rand::{distributions::Uniform, prelude::*};

fn main() {
    const n: usize = 20;

    let mut rng = thread_rng();
    let dist = Uniform::from(0..10000);
    let arr: Vec<i32> = (0..n).map(|_| dist.sample(&mut rng)).collect();



    println!("{:?}", arr);
}
