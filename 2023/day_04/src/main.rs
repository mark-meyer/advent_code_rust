use std::path::Path;
use std::fs;

use day_04::{solve_part_one, solve_part_two};

fn main() {
    let p = Path::new("data.txt");
    let input = fs::read_to_string(p).expect("can't read file!");
    let input: Vec<_> = input.lines().collect();

    let solution_part_one = solve_part_one(&input);
    println!("Part One: {:?}", solution_part_one);

    let solution_part_two = solve_part_two(&input);
    println!("Part Two: {:?}", solution_part_two);
}
