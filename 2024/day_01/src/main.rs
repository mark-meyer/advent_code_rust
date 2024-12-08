use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn parse_input(file: File) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let reader = BufReader::new(file);

    let (mut col1, mut col2):(Vec<_>, Vec<_>) = reader
    .lines()
    .filter_map(Result::ok)
    .filter_map(|line| {
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            Some((left.parse::<i32>().ok()?, right.parse::<i32>().ok()?))
        } else {
            None
        }
    })
    .unzip();
    col1.sort();
    col2.sort();
    
    Ok((col1, col2))

}

fn main() {
    let path = Path::new("data.txt");
    let file = File::open(path).expect("Can't open file!");
    let (col1, col2) = parse_input(file).expect("Coulldn't parse file");

    //  Part One
    let total:i32 = col1.iter()
    .zip(col2.iter())
    .map(|(a, b)| (a-b).abs())
    .sum();
    println!("Part One: {}", total);

    // Part Two    
    let counts:HashMap<i32, i32> = col2.iter().fold(HashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    let total:i32 = col1.iter()
    .map(|n| n * counts.get(n).unwrap_or(&0))
    .sum();

    println!("Part Two: {:?}", total)
}
