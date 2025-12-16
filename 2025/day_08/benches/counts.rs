use criterion::{ black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;

use day_08::{parse_file, part_two, part_one, make_kd_tree};
fn criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)               // fewer samples
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3))
}

fn bench_part_two(c: &mut Criterion) {
    let f = File::open("data.txt").unwrap();
    let data = parse_file(f).expect("parsing file");
    let tree = make_kd_tree(&data);

    c.bench_function("make tree", |b| {
        b.iter(|| {
            let removed = make_kd_tree(black_box(&data));
            black_box(removed);
        })
    });

    c.bench_function("part_two", |b| {
        b.iter(|| {
            let removed = part_two(black_box(&tree), 1000);
            black_box(removed);
        })
    });

    c.bench_function("part_one", |b| {
        b.iter(|| {
            let removed = part_one(black_box(&tree), 1000);
            black_box(removed);
        })
    });
}



criterion_group!{
    name = bench_group;
    config = criterion();
    targets = bench_part_two
}

criterion_main!(bench_group);