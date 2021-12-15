use std::fs;
use std::path::Path;
use std::collections::HashMap;
use fourteen::PolymerCounter;

const PATH: &str = "./data.txt";

fn parse_data(data: &str) -> (&str, HashMap<&str, &str>) {
    let mut lines = data.lines();
    let template:&str = lines.next().unwrap();

    let lookup = HashMap::from_iter(
        lines.skip(1)
        .map(|line| line.split_once(" -> ").unwrap())
    );
    (template, lookup)
}


fn main() {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("Could not read data");
    let (template, lookup) = parse_data(&data);

    // Part One
    let mut counter = PolymerCounter::new(template, &lookup);
    counter.run_substitutions(10);
    let total = counter.count_letters();
    println!("Solution 1: {}", total);

    // Part Two
    let mut counter = PolymerCounter::new(template, &lookup);
    counter.run_substitutions(40);
    let total = counter.count_letters();
    println!("Solution 2: {}", total);
}
