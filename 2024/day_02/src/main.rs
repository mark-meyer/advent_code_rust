use std::path::Path;
use std::fs::File;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn parse_error<E: std::fmt::Display>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{}", e))
}


fn parse_file(f:File) -> std::io::Result<Vec<Vec<i32>>> {
    let reader = BufReader::new(f);

    reader
    .lines()
    .map(|line| {
        let line = line?;
        line.split_whitespace()
        .map(|n| n.parse::<i32>().map_err(parse_error)).collect()
        })
    .collect()
}

fn is_valid(v: &[i32]) -> bool {
    let up = HashSet::from([1, 2, 3]);
    let down = HashSet::from([-1, -2, -3]);
    
    let h:HashSet<i32> = v.windows(2).map(|s| s[0] - s[1]).collect();
    h.is_subset(&up) || h.is_subset(&down)
    
}

fn main() {
    let p = Path::new("data.txt");
    let f = File::open(p).expect("couldn't open file");
    let data = parse_file(f).unwrap();

    // Part One

    let count_valid = data.iter().filter(|line| is_valid(&line)).count();
    println!("Part one: {}", count_valid);

    // Part Two

    let valid_counts = data.iter()
    .filter(|&line| {
        line.iter()
            .enumerate()
            .any(|(i, _)| {
                let modified_line: Vec<i32> = line.iter().enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &val)| val)
                    .collect();

                is_valid(&modified_line)
            })
    })
    .count();
    println!("Part one: {}", valid_counts);
}
