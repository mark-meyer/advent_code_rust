use std::fs::File;
use day_04::{parse_input, part_one, part_two};

fn main() {
    let f = File::open("data.txt").expect("Could not open the file!");
    let mut grid = parse_input(f);
    
    println!("part one {}", part_one(&grid));
    println!("part two {}", part_two(&mut grid))
}
