use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

fn parse_data(file: File) -> Result<Vec<i32>, Box<dyn Error>> {
    let reader = BufReader::new(file);

    reader
    .lines()
    .map(|line| {
        let line = line?;
        let mut chars = line.chars();
        let mult = match chars.next() {
            Some('R') => 1,
            Some('L') => -1,
            _ => return Err(format!("Incorrect format for line {}", line).into())
        };
        let slc:i32 = chars.as_str().parse().expect("Could not parse input");
        Ok(mult * slc)
    })
    .collect()
}

fn part_one(data: &[i32]) -> i32 {
    let mut current = 50;
    let mut zeros = 0;
    for n in data.iter() {
        current = (current + n) % 100;
        if current == 0 {
            zeros += 1
        }
    }
    zeros 
}

fn part_two(data: &[i32]) -> i32 {
    let mut current = 50;
    let mut zeros = 0;
    for n in data.iter() {
        // println!("current: {}", current);
        let (rotations, mut rem) = (n.abs() / 100, n.abs() % 100);
        if *n < 0 {
            rem *= -1;
        }
        zeros += rotations;
        if (current != 0) && ((current + rem) <= 0 || (current + rem) >= 100) {
            zeros += 1;
        }
        // Careful: Rust's % doesn't work like python with negative numbers
        current = (current + n).rem_euclid(100);
    }
    zeros
}

fn main() {
    let file = File::open("data.txt").expect("Couldn't open file!");
    let data = parse_data(file).expect("Could not parse file");

    let part_one_answer = part_one(&data);
    println!("Part One: {}", part_one_answer);

    let part_two_answer = part_two(&data);
    println!("Part Two: {}", part_two_answer);
}
