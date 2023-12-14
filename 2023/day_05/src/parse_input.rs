use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use super::Conversion;

pub fn parse_file(filename: &str) -> io::Result<(Vec<i64>, Vec<Vec<Conversion>>)> {
    let p = Path::new(filename);
    let f = File::open(p).expect("Could not open the file!");
    let mut reader = BufReader::new(f).lines();
    
    let seeds: String = reader.next().unwrap()?;
    let seeds:Vec<i64> = seeds.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    reader.next();

    let mut group: Vec<Vec<Conversion>> = Vec::new();
    let mut conversions: Vec<Conversion> = Vec::new();
    
    for line in reader {
        let line = line.unwrap();
        
        if line.chars().any(|c| c.is_alphabetic()) {
            continue;
        }

        if line.is_empty() {
            if !conversions.is_empty() {
                group.push(conversions);
                conversions = Vec::new();
            }
            continue;
        }
        
        conversions.push(Conversion::from(line));
    }
    if !conversions.is_empty() {
        group.push(conversions);
    }

    Ok((seeds, group))
}