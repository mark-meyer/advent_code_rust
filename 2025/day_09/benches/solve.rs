use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;

use day_09::{solve_compression, parse_input, part_one};

fn criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)               // fewer samples
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3))
}

fn bench_part_two(c: &mut Criterion) {
    // excluded from timing
    let f = File::open("data.txt").unwrap();
    let poly = parse_input(f).unwrap();

    c.bench_function("part one", |b| {
        b.iter(|| {
            // clone per-iteration so each run starts from same state
            part_one(black_box(&poly));
        })
    });
    c.bench_function("part_two", |b| {
        b.iter(|| {
            // clone per-iteration so each run starts from same state
            solve_compression(black_box(&poly));
        })
    });
}

criterion_group!{
    name = benches;
    config = criterion();
    targets = bench_part_two
}

criterion_main!(benches);
