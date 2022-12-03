use std::fs;
use std::path::Path;
use nine::make_matrix;
use nine::solutions;

static PATH: &'static str = "./data.txt";

fn main() {
    let path = Path::new(PATH);
    let raw_data = fs::read_to_string(path).expect("You file was lost in the smoke");
    let matrix = make_matrix(&raw_data);

    let (solution_one, solution_two) = solutions(&matrix);
    println!("Solution 1: {:?}", solution_one);
    println!("Solution 2: {:?}", solution_two);

}

