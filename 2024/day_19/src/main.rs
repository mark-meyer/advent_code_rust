use std::fs;
use day_19::*;

fn parse(s:&str) -> (Trie, Vec<String>) {
    let (patterns, designs) = s.split_once("\n\n").unwrap();
    let mut prefixes = Trie::new();
    for p in patterns.split(", ") {
        prefixes.insert(p.trim());
    };
    let designs =  designs.split("\n")
    .map(|line| line.to_owned())
    .collect();

    (prefixes, designs)
}

fn part_one<'a>(prefixes: &Trie, towels: &[String]) -> usize {
    towels
    .iter()
    .filter(|t| prefixes.is_possible(t))
    .count()

}

fn main() {
    let s = fs::read_to_string("data.txt").expect("we lost the designs file!");
    let (prefixes, designs) = parse(&s);
    println!("{:?}", part_one(&prefixes, &designs));
    println!("{:?}", prefixes.is_possible("gwwgwbgbgbuugwurgggwrubrruuwgbwgwrgwrbwrugwwrrugrwgu"));
}
