use std::path::Path;
use std::fs;
use ten::*;

const PATH: &str = "./data.txt";

fn main() {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("Can't find file");

    let solution1 = solution_one(&data);
    println!("solution one: {:?}", solution1);

    let solution2 = solution_two(&data);
    println!("solution one: {:2}", solution2);

}
