use std::fs;
use std::path::Path;

use day_03::*;

fn main() {
    let path = Path::new("data.txt");
    let data = fs::read_to_string(path).expect("could not open the input file");
    let lines:Vec<&str> = data.lines().collect();
    
    let day_one_answer = part_one(&lines);
    println!("Day One: {:?}", day_one_answer);

    let day_two_answer = part_two(&lines);
    println!("Day Two: {:?}", day_two_answer);

}
