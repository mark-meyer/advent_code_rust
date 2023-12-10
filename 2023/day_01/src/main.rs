use std::fs;
use std::path::Path;

use day_01::*;

static DATA: &str = "data.txt";

fn main() {
    let p = Path::new(DATA);
    let input = fs::read_to_string(p).expect("Can't open file!");
    
    let part_one_answer = solve_part_one(&input);    
    let part_two_answer = solve_part_two(&input);

    println!("Part One: {:?}", part_one_answer);
    println!("Part Two {}", part_two_answer);
}
