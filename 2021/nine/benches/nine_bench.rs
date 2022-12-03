//! Benchmarks
use std::fs;
use std::path::Path;

#[macro_use]
extern crate bencher;
extern crate nine;
use self::bencher::Bencher;

use nine::make_matrix;
use nine::solutions;


static PATH: &'static str = "./data.txt";


fn bench_test(b: &mut Bencher) {
    let path = Path::new(PATH);
    let raw_data = fs::read_to_string(path).expect("You file was lost in the smoke");
    let matrix = make_matrix(&raw_data);

    b.iter(|| solutions(&matrix))
}

benchmark_group!(
    benches,
    bench_test,
);
benchmark_main!(benches);
