use day_19::*;
use std::fs;

fn parse(s: &str) -> (Trie, Vec<String>) {
    let (patterns, designs) = s.split_once("\n\n").unwrap();
    let mut prefixes = Trie::new();
    for p in patterns.split(", ") {
        prefixes.insert(p.trim());
    }
    let designs = designs.split("\n").map(|line| line.to_owned()).collect();

    (prefixes, designs)
}

fn part_one(prefixes: &Trie, towels: &[String]) -> usize {
    towels.iter().filter(|t| prefixes.is_possible(t)).count()
}
fn part_two(prefixes: &Trie, towels: &[String]) -> u64 {
    towels.iter().map(|t| prefixes.count_possible(t)).sum()
}

fn main() {
    let s = fs::read_to_string("data.txt").expect("we lost the designs file!");
    let (prefixes, designs) = parse(&s);
    println!("{:?}", part_one(&prefixes, &designs));
    println!("{:?}", part_two(&prefixes, &designs));
}
