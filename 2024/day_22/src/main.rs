use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::error::Error;
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

fn part_two(numbers: &[SecretNumber]) -> ((i64, i64, i64, i64), i64) {
    let mut global_counts = HashMap::new();
    for number in numbers.iter() {
        number.add_prices(2000, &mut global_counts);
    }
    global_counts.into_iter().max_by_key(|entry| entry.1).unwrap()
}


fn main() {
    let f = File::open("data.txt").expect("Couldn't open the file");
    let nums = parse_input(f).unwrap();

    println!("{:?}", part_one(&nums));
    let (_diffs, price) = part_two(&nums);
    println!("Part Two {:?}", price);
}
