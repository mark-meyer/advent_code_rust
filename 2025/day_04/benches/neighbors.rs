use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;

use day_04::{parse_input, part_two};

fn criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)               // fewer samples
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3))
}

fn bench_part_two(c: &mut Criterion) {
    // excluded from timing
    let f = File::open("data.txt").unwrap();
    let grid0 = parse_input(f);

    c.bench_function("part_two", |b| {
        b.iter(|| {
            // clone per-iteration so each run starts from same state
            let mut g = grid0.clone();
            let removed = part_two(black_box(&mut g));
            black_box(removed);
        })
    });
}

criterion_group!{
    name = benches;
    config = criterion();
    targets = bench_part_two
}

criterion_main!(benches);
