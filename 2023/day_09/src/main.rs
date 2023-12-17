use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use day_09::*;

fn parse_input(file: File) -> Vec<Vec<i32>>{
    let lines = BufReader::new(file).lines();
    lines.map(|l| {
        l.unwrap()
        .split_whitespace()
        .flat_map(|n| n.parse())
        .collect()
    }).collect()
}

fn main() {
    let path = Path::new("data.txt");
    let file = File::open(path).expect("Can't open file!");
    let input = parse_input(file);

    let transformed:Vec<Vec<Vec<i32>>> = input
        .into_iter()
        .map(|line| transform_to_zero(line))
        .collect();

    let part_one_result:i32 = transformed
        .iter()
        .map(|v| transform_from_zeros(&v))
        .sum();   

    println!("Part One: {}", part_one_result);

    let part_two_result:i32 = transformed.iter()
        .map(|v| transform_prefix(&v))
        .sum();   

    println!("Part Two: {}", part_two_result);
}
