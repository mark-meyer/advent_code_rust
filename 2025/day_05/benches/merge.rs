use criterion::{BatchSize, black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;

use day_05::{parse_file, parse_file_raw, part_two, merge_ranges};

fn criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)               // fewer samples
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3))
}


fn benches(c: &mut Criterion) {
    // excluded from timing
    let f = File::open("data.txt").unwrap();
    let (merged_ranges, _) = parse_file(f).expect("parse file");

    let f2 = File::open("data.txt").unwrap();
    let (raw_ranges, _) = parse_file_raw(f2).expect("parse raw");
   
    c.bench_function("merge_ranges", |b| {
        b.iter_batched( 
            || raw_ranges.clone(), 
            |v| black_box(merge_ranges(v)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("part_two (premerged", |b| {
        b.iter_batched_ref(
            || merged_ranges.clone(),
            |v| black_box(part_two(black_box(v))),
            BatchSize::SmallInput)
    });

    c.bench_function("merge_ranges + part 2", |b| {
        b.iter_batched(
            || raw_ranges.clone(),
            |v| {
                let merged = merge_ranges(v);
                black_box(part_two(black_box(&mut merged.clone())))
            },
            BatchSize::SmallInput);
    });
}

criterion_group!{
    name = bench_group;
    config = criterion();
    targets = benches
}

criterion_main!(bench_group);