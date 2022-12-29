use std::path::Path;
use std::fs;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn find_unique(s: &str) -> char {
    let mid = s.len() / 2;
    let (a, b) = s.split_at(mid);

    let s1 = HashSet::<char>::from_iter(a.chars());
    let s2 = HashSet::<char>::from_iter(b.chars());

    let inter = s1.intersection(&s2);
    inter.into_iter().next().unwrap().clone()
}

fn find_unique_in_three(f: fs::File) -> Vec<char> {
    let mut lines = BufReader::new(f).lines();

    let mut chars = vec![];
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        let mut s1 = HashSet::<char>::from_iter(l1.unwrap().chars());
        let s2 = HashSet::<char>::from_iter(l2.unwrap().chars());
        let s3 = HashSet::<char>::from_iter(l3.unwrap().chars());
        
        s1.retain(|item| s2.contains(item) && s3.contains(item));
        chars.push(s1.into_iter().next().unwrap().clone());
    }
    
    chars
}

fn priority(c: &char) -> u32{
    match c.is_ascii_lowercase() {
        true => *c as u32 - 96,
        false => *c as u32 - 38
    }
}

fn main() {
    let path = Path::new("data.txt");
    let file = fs::File::open(path).expect("Whoops, where's the file?");

    let total: u32 = BufReader::new(file).lines()
    .map(|line| line.unwrap().to_string())
    .map(| s | find_unique(&s) )
    .map(| c | priority(&c))
    .sum();

    println!("Part One {}",total);

    let file = fs::File::open(path).expect("Whoops, where's the file?");
    let c = find_unique_in_three(file);
    let total: u32 = c.iter().map(| c | priority(&c)).sum();
    println!("Part Two {:?}", total);
}
