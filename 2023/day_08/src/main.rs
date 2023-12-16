use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use num::integer::lcm;

use day_08::*;

fn parse_input(filename: &str) -> Result<(Vec<Instruction>, Graph), io::Error> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    
    let instructions: Vec<Instruction> = lines.next().unwrap()?.chars().map(|c| c.into()).collect();
    let mapping = create_graph(lines.skip(1)).unwrap();

    Ok((instructions, mapping))
}

fn main() {
    let (instructions, mapping) = parse_input("data.txt").expect("Can't parse file!");

    // Part One
    let instruction_iter = instructions.iter().cycle();
    let f = |start: &str| start != "ZZZ";
    let steps = get_path_length(f, &mapping, "AAA", instruction_iter);
    
    println!("Part One: {:?}", steps);

    // Part Two
    let f = |start: &str| !start.ends_with("Z");
    let result:u64 = mapping
        .keys()
        .filter(|k| k.ends_with("A")).map(|k| {
            get_path_length(f, &mapping, k, instructions.iter().cycle())
        })
        .reduce(lcm)
        .unwrap();

    println!("Part Two: {:?}", result);
}
