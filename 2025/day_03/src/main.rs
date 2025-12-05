use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(f: File) -> Vec<Vec<u32>> {
    BufReader::new(f)
    .lines()
    .flat_map(|l| l)
    .map(|line| line.chars()
        .flat_map(|c| c.to_digit(10))
        .collect())
    .collect()
}

fn high_joltage(line:&[u32], length:usize) -> u64 {
    let mut total = 0;
    let mut l_index = 0;
    for r_index in (0 .. length).rev(){
        let segment = &line[l_index..line.len() - r_index];
        if let Some((i, max)) = segment.iter().enumerate().max_by_key(|p| (p.1, -(p.0 as isize))) {
            l_index = l_index + i + 1;
            total += 10_u64.pow(r_index as u32) * (*max as u64)
        }
    }
    total
}

fn part_one(data:&[Vec<u32>]) -> u64 {
    data.iter().map(|line| high_joltage(&line, 2)).sum()
}


fn part_two(data:&[Vec<u32>]) -> u64 {
    data.iter().map(|line| high_joltage(&line, 12)).sum()
}

fn main() {
    let file = File::open("data.txt").expect("Could not open battery bank!");
    let data = parse_file(file);
    println!("Part one: {:?}", part_one(&data));
    println!("Part two: {:?}", part_two(&data));
}


#[cfg(test)]
mod test {
use super::*;

#[test]
fn test_joltage(){
    assert_eq!(high_joltage(&vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 12), 987654321111);
    assert_eq!(high_joltage(&vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 2), 98);
    assert_eq!(high_joltage(&vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 12), 811111111119);
    assert_eq!(high_joltage(&vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 2), 89);
    assert_eq!(high_joltage(&vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 12), 434234234278);
    assert_eq!(high_joltage(&vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 2), 78);
    assert_eq!(high_joltage(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 12), 888911112111);
    assert_eq!(high_joltage(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 2), 92);

}

}