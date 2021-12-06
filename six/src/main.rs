use std::path::Path;
use std::fs;
use std::collections::VecDeque;

static PATH: &'static str = "./data.txt";

fn parse_file(path: &str) -> Vec<usize> {
    let path = Path::new(path);
    let data = fs::read_to_string(path).expect("The laternfish ate you data file");
    data.trim().split(',').map(|n| n.parse().unwrap()).collect()
}

fn count_fish(days: usize, data: &[usize]) -> i64 {
    let mut counts: VecDeque<i64> = [0; 9].into();
    for &index in data {
        counts[index] += 1;
    }
    for _ in 0..days {
        counts[7] += counts[0];
        counts.rotate_left(1);
    }
    counts.iter().sum()
}

fn main() {
    let numbers = parse_file(PATH);
    println!("Solution1: {}", count_fish(80, &numbers));
    println!("Solution1: {}", count_fish(256, &numbers));
}
