use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::error::Error;
use rayon::prelude::*;

use day_22::*;

fn parse_input(f:File) -> Result<Vec<SecretNumber>, Box<dyn Error>> {
    BufReader::new(f)
    .lines()
    .map(|line| {
        SecretNumber::try_from(line?)
        .map_err(|_| "Bad number!".into())
    })
    .collect()
}

fn part_one(numbers: &[SecretNumber]) -> i64 {
    numbers
        .iter()
        .flat_map(|n| n.skip(2000).take(1))
        .sum()
}

fn part_two(numbers: &[SecretNumber]) -> ((i8, i8, i8, i8), i64) {
    let global_counts = numbers
    .par_iter()
    .map(|number| number.add_prices(2000))
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
