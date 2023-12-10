use std::fs;
use std::path::Path;

use day_02::*;


fn main() {
    let p = Path::new("data.txt");
    let f = fs::read_to_string(p).expect("Could not read file");

    println!("Part One: {}", solve_part_one(f.lines()));
    println!("Part One: {}", solve_part_two(f.lines()));

}
