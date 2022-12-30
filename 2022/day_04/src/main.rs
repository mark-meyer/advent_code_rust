use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::num::ParseIntError;

static PATH: &str = "data.txt";

type Pair = (usize, usize);
#[derive(Debug, Clone)]
struct FileParseError;

fn range_pair(s: &str) -> Result<Pair, ParseIntError> {
    let (start, end) = s.split_once('-').unwrap();
    Ok((start.parse()?, end.parse()?))
}

fn create_pairs(s: &str) -> Result<(Pair, Pair), FileParseError> {
    let (elf1, elf2) = s.split_once(",").ok_or(FileParseError)?; 
    range_pair(&elf1)
    .and_then(|r1| range_pair(&elf2).map(|r2| (r1, r2)))
    .map_err(|_| FileParseError)
}

fn total_overlap(r1: Pair, r2: Pair) -> bool {
    (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1)
}
fn partial_overlap(r1: Pair, r2: Pair) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}


fn main() {
    let path = Path::new(PATH);
    let file = File::open(path).expect("No file?");

    let ranges:Vec<(Pair, Pair)> = BufReader::new(file)
    .lines()
    .map(|line| create_pairs(&line.unwrap()).expect("could not parse input"))
    .collect();

    let total = ranges.iter().filter(|pair|  total_overlap(pair.0, pair.1)).count();
    println!("Part One: {}", total);

    let total = ranges.iter().filter(|pair|  partial_overlap(pair.0, pair.1)).count();
    println!("Part Two: {}", total);

}
