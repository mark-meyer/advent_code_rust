use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use fifteen::*;

const PATH: &str = "./data.txt";

fn parse_input<R:Read>(input: R) -> Vec<Vec<usize>>{
    let input = BufReader::new(input);
    input.lines().map( |line|
        line.unwrap().trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
    ).collect()
}

fn main() {
    let path = Path::new(PATH);
    let file = File::open(path).expect("Your file got lost in the cave");
    let matrix = parse_input(file);
    let g = Graph::new(&matrix);
    let w = matrix[0].len();
    let h = matrix.len();

    println!("solution 1: {:?}", g.dijkstra((0, 0), (w-1, h-1)));

    let biggy = make_large_graph(&matrix, 5);
    let g2 = Graph::new(&biggy);
    let w = biggy[0].len();
    let h = biggy.len();
    println!("solution 2: {:?}", g2.dijkstra((0, 0), (w-1, h-1)));

}
