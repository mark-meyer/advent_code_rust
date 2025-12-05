use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;


pub fn part_one(ranges: &Vec<Range>, ingredients: &Vec<u64>) -> u64 {
    ingredients
    .iter()
    .fold(0, |acc, &ing| acc + ranges.iter().any(|range| range.includes(ing)) as u64)
}

pub fn part_two(ranges: &Vec<Range>) -> u64 {
    ranges
    .iter()
    .fold(0, |acc, range| acc + range.end - range.start + 1)
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Range {
    pub start: u64,
    pub end: u64
}

impl Range {
    pub fn includes(&self, n: u64) -> bool {
        return n >= self.start && n <= self.end
    }
}


pub fn parse_file_raw(f: File) -> Result<(Vec<Range>, Vec<u64>), Box<dyn Error>> {
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in lines.by_ref() {
        let line = line?;

        if line.is_empty() {
            break; 
        }
        if let Some((start, end)) = line.split_once("-"){
            let start = start.parse()?;
            let end = end.parse()?;
            ranges.push(Range {start, end});
        } 
    }
    for line in lines {
        let line = line.expect("read line");
        let ingredient = line.parse()?;
        ingredients.push(ingredient);

    }
    Ok((ranges, ingredients))
}

pub fn parse_file(f: File) -> Result<(Vec<Range>, Vec<u64>), Box<dyn Error>> {
    // separate this out to make benchmarking a little more fun

    let (ranges, ingredients) = parse_file_raw(f)?;
    // merge_ranges will merge and sorted the ranges
    let merged = merge_ranges(ranges);
    Ok((merged, ingredients))
}

pub fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_unstable_by_key(|r| r.start);

    let mut merged = Vec::with_capacity(ranges.len());

    let mut range_iter = ranges.into_iter();
    let mut current = range_iter.next().unwrap();
    
    for next in range_iter {
        if next.start <= current.end {
            if next.end > current.end {
                current.end = next.end;
            }
        }
        else {
            merged.push(current);
            current = next
        }
    }
    merged.push(current);
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge(){
        // barely overlapping
        let ranges = vec![Range{start:2, end: 10}, Range{start:10, end: 12}];
        assert_eq!(vec![Range{start:2, end: 12}], merge_ranges(ranges));

        // overlapping
        let ranges = vec![Range{start:2, end: 10}, Range{start:3, end: 12}];
        assert_eq!(vec![Range{start:2, end: 12}], merge_ranges(ranges));

        // contained
        let ranges = vec![Range{start:2, end: 10}, Range{start:3, end: 8}];
        assert_eq!(vec![Range{start:2, end: 10}], merge_ranges(ranges));

        // Non overlapping
        let ranges = vec![Range{start:2, end: 10}, Range{start:13, end: 18}];
        assert_eq!(vec![Range{start:2, end: 10}, Range{start:13, end: 18}], merge_ranges(ranges));
    }

    #[test]
    fn test_includes() {
        let range = Range {start: 10, end: 20 };

        assert!(range.includes(10));
        assert!(range.includes(20));
        assert!(range.includes(15));

        assert!(!range.includes(9));
        assert!(!range.includes(21));

    }
}