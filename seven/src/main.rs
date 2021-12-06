use std::fs;
use std::path::Path;

static PATH: &'static str = "./data.txt";

fn parse_data(data:&str) -> Vec<i64> {
    data.split(',').map(|n| n.parse().unwrap()).collect()
}

fn problem_one(data:&Vec<i64>) -> i64 {
    let med = data[data.len() / 2];
    data.iter().fold(0, |a, n| a + (n - med).abs())
}

fn tri_sum(n: i64) -> i64 {
    (n * (n + 1)) / 2
}

fn p2_cost(pos: i64, data:&Vec<i64>) -> i64 {
    data.iter().fold(0, |a, n| a + tri_sum((n-pos).abs()))
}

fn problem_two(data:&Vec<i64>) -> i64 {
    let max_val = data.iter().max().unwrap();
    (0..*max_val).map(|pos| p2_cost(pos, data)).min().unwrap()
}


fn main() {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("Crabs ran out of fuel!");
    // sort for convenience
    let mut data = parse_data(&data);
    data.sort();

    println!("Solution 1: {:?}", problem_one(&data));
    println!("Solution 2: {:?}", problem_two(&data));
}
