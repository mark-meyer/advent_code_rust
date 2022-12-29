use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead};


const PATH: &str = "./data.txt";

fn main() {
    let path = Path::new(PATH);
    let f = fs::File::open(path).expect("whoops file not found");
    let buffered = BufReader::new(f);

    let  mut totals:Vec<u32> = vec![];
    let  mut current: u32 = 0;

    for line in buffered.lines() {
        let s = line.unwrap();

        if s == "" {
            totals.push(current);
            current = 0;
        } else {
            current += s.parse::<u32>().unwrap();
        }
    }


    totals.sort_by(|a, b| b.cmp(a));
    println!("part one {}", totals[0]);
    println!("part two: {}", totals.iter().take(3).fold(0, |a, b| a+b));
}