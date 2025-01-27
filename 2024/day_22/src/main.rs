use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::error::Error;
use rayon::prelude::*;

use day_22::*;

fn parse_input(f:File) -> Result<Vec<i64>, Box<dyn Error>> {
    BufReader::new(f)
    .lines()
    .map(|line| {
        line?.parse()
        .map_err(|_| "Bad number!".into())
    })
    .collect()
}

fn part_one(numbers: &[i64]) -> i64 {
    numbers
        .iter()
        .map(|n| nth_next(*n, 2000))
        .sum()
}

fn part_two(numbers: &[i64]) -> ((i8, i8, i8, i8), i64) {
    let global_counts = numbers
    .par_iter()
    .map(|number| add_prices(*number, 2000))
    .reduce(|| HashMap::new(),
        |mut acc, local_counts| {
            for (diffs, price) in local_counts {
                *acc.entry(diffs).or_insert(0_i64) += price as i64;
            }
            acc
        }
    );
    global_counts.into_iter().max_by_key(|entry| entry.1).unwrap()
}


fn main() {
    let f = File::open("data.txt").expect("Couldn't open the file");
    let nums = parse_input(f).unwrap();

    println!("{:?}", part_one(&nums));
    let (_diffs, price) = part_two(&nums);
    println!("Part Two {:?}", price);
}
