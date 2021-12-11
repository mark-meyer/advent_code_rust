//! Benchmarks
use std::fs;
use std::path::Path;

#[macro_use]
extern crate bencher;
extern crate ten;
use self::bencher::Bencher;

use ten::solution_one;
use ten::solution_two;


static PATH: &'static str = "./data.txt";


fn bench_test_one(b: &mut Bencher) {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("You file was lost in the smoke");

    b.iter(|| solution_one(&data))
}

fn bench_test_two(b: &mut Bencher) {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("You file was lost in the smoke");

    b.iter(|| solution_two(&data))
}

benchmark_group!(
    benches,
    bench_test_one,
    bench_test_two
);

benchmark_main!(benches);
