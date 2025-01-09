use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};
use regex::Regex;

use day_07::*;


fn parse_input(f: File) -> Result<Vec<TestValue>, Box<dyn Error>> {
    let re = Regex::new(r"\d+")?;

    BufReader::new(f)
    .lines()
    .map(|line_result| {
        let line = line_result?;

        let numbers = re
        .find_iter(&line)
        .map(|m| m.as_str().parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;

        let target = numbers.get(0).ok_or("Not enough numbers!")?;
        let values = numbers[1..].to_vec();
        
        Ok(TestValue::new(*target, values))
    })
    .collect()
}


fn concat(a:u64, b:u64) -> u64 {
    if b == 0 {
        return a * 10
    }
    a * 10_u64.pow(b.ilog10() + 1) + b
}

fn run<F>(tests: &Vec<TestValue>, operators: &Vec<F>) -> u64
    where F: Fn(u64, u64) -> u64
{
    tests.iter()
    .filter_map(|t| t.valid_total(operators))
    .sum()
}

fn main() {
    let p = Path::new("data.txt");
    let f = File::open(p).expect("The elephants took the file too!");
    let input_values = parse_input(f).expect("Could not parse file");
    
    let mut operators = vec![
        Add::add,
        Mul::mul,
    ];

    println!("Part one: {:?}", run(&input_values, &operators));
    
    operators.push(concat);
    println!("Part two: {:?}", run(&input_values, &operators));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(123, 456), 123456);
        assert_eq!(concat(123, 0), 1230);
        assert_eq!(concat(1, 1230), 11230);
        assert_eq!(concat(0, 0), 0);
        assert_eq!(concat(0, 123), 123);
    }
}