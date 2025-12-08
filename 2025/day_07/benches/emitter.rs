use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

use day_07::{run_transporter, parse_file};

fn criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)               // fewer samples
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3))
}

fn bench_emmiter(c: &mut Criterion) {
    // excluded from timing
    let raw_input = read_to_string("data.txt").unwrap();
    let manifold = parse_file(&raw_input);

    c.bench_function("emitter", |b| {
        b.iter(|| {
            black_box(run_transporter(black_box(&manifold)));
        })
    });
}

criterion_group!{
    name = benches;
    config = criterion();
    targets = bench_emmiter
}

criterion_main!(benches);
