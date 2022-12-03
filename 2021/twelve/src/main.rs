use std::path::Path;
use std::fs;
use twelve::*;

const PATH: &str = "./data.txt";

fn main() {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("file not found...maybe it entered a small cave twice");
    let graph = parse_input(&data);

    let solution_one = count_paths_one(&graph, "start", None);
    println!("solution one: {}", solution_one);

    let solution_two = count_paths_two(&graph, "start", None, false);
    println!("solution two: {}", solution_two);
}
