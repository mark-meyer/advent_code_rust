use std::fs;
use std::error::Error;
use std::collections::HashSet;

use day_02::*;


fn parse_input(s: &str) -> Vec<Range>{
    s.split(',')
    .flat_map(|p| p.split_once('-'))
    .map(|r| Range::from(r))
    .collect()
}

fn part_one(pairs: &[Range]) -> Result<usize, Box<dyn Error>>{
    pairs.iter()
    .flat_map(|p| p.adjusted_range(2))
    .try_fold(HashSet::new(), |mut acc, pair| {
            acc.extend(pair.invalid_keys()?); 
            Ok(acc)
        })
    .map(|invalids| invalids.iter().sum())
}


fn part_two(pairs: &[Range]) -> Result<usize, Box<dyn Error>>{
    // Build into a set to avoid double counting keys like
    // 222222, which can be split in more than one way.
    let mut invalids = HashSet::new();
    for pair in pairs.iter() {
        for l in 2..pair.end.len() + 1 {
            if let Some(adjusted) = pair.adjusted_range(l) {
                invalids.extend(adjusted.invalid_keys()?);
            }
        }
    }
    return Ok(invalids.iter().sum())
}
            
fn main() {
    let raw_data = fs::read_to_string("data.txt").expect("Could not read file!");
    let data = parse_input(&raw_data);

    let part_one_answer = part_one(&data).expect("Error finding keys!");
    println!("Part One {}", part_one_answer);

    let part_two_answer = part_two(&data).expect("Error finding keys!");
    println!("Part Two {}", part_two_answer);
}
