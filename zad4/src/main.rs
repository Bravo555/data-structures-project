use std::{fs, io::Write, time::Instant};

use rand::{prelude::SmallRng, SeedableRng};
use zad4::*;

fn main() {
    let rng = SmallRng::seed_from_u64(0);

    let config_str = fs::read_to_string("config.ini").expect("nie podano pliku konfiguracyjnego");
    let mut config = config_str.lines().filter(|line| !line.starts_with(";"));

    let mut sizes = config
        .next()
        .expect("start not provided")
        .split_whitespace()
        .filter_map(|word| word.parse().ok());
    let densities: Vec<_> = config
        .next()
        .expect("start not provided")
        .split_whitespace()
        .filter_map(|word| word.parse::<f32>().ok())
        .collect();
    let repetitions = config
        .next()
        .expect("start not provided")
        .parse()
        .expect("error parsing repetitions");

    let start = sizes.next().expect("start not provided");
    let end = sizes.next().expect("end not provided");
    let step = sizes.next().expect("step not provided");

    let sizes = (start..=end).step_by(step);
    let mut output = fs::File::create("results.csv").unwrap();
    write!(
        output,
        "gestosc,wielkosc instancji,macierz sasiedztwa,lista sasiedztwa,pek wyjsciowy,macierz incydencji\n",
    )
    .unwrap();

    for size in sizes {
        for &density in &densities {
            println!("Generating graphs...\n");
            let adj_matrix = AdjMatrix::random_connected(size, density, &mut rng.clone());
            let list = AdjList::random_connected(size, density, &mut rng.clone());
            let inc_matrix = IncidenceMatrix::random_connected(size, density, &mut rng.clone());
            let bundle = Bundle::random_connected(size, density, &mut rng.clone());

            println!("benchmarking for size: {}", size);
            println!("density: {}", density);
            let start = Instant::now();
            for _ in 0..repetitions {
                let _dijkstra = adj_matrix.dijkstra(0);
            }
            let time_matrix = Instant::elapsed(&start);
            println!("Macierz sasiedztwa:\t{:?}", &time_matrix);

            let start = Instant::now();
            for _ in 0..repetitions {
                let _dijkstra = list.dijkstra(0);
            }
            let time_list = Instant::elapsed(&start);
            println!("Lista sasiedztwa:\t{:?}", &time_list);

            let start = Instant::now();
            for _ in 0..repetitions {
                let _dijkstra = bundle.dijkstra(0);
            }
            let time_bundle = Instant::elapsed(&start);
            println!("Pek wyjsciowy:\t\t{:?}", &time_bundle);

            let start = Instant::now();
            for _ in 0..repetitions {
                let _dijkstra = inc_matrix.dijkstra(0);
            }
            let time_inc = Instant::elapsed(&start);
            println!("Macierz incydencji:\t{:?}\n", &time_inc);

            writeln!(
                output,
                "{},{},{},{},{},{},{},{}",
                density,
                size,
                time_matrix.as_millis(),
                adj_matrix.memory(),
                time_list.as_millis(),
                list.memory(),
                time_bundle.as_millis(),
                bundle.memory(),
            )
            .unwrap();
        }
    }
}
