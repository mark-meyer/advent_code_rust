use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use day_07::*;

fn parse_input(filename: &str) -> Vec<Hand> {
    let path = Path::new(filename);
    let file = File::open(path).expect("Could not read file");
    let buffer = BufReader::new(file);
  
    buffer.lines()
    .map(|line| Hand::from(line.unwrap()))
    .collect()
}

fn main() {
    let mut hands = parse_input("data.txt");
    hands.sort();
    
    let part_one = solutions::part_one(&mut hands);
    println!("Part One: {}", part_one);

    let part_two = solutions::part_two(&mut hands);
    println!("Part Two: {}", part_two);

}
