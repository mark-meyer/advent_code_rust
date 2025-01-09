use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};

use regex::Regex;

#[derive(Debug)]
pub struct TestValue {
    pub target: u64,
    pub values: Vec<u64>,
}

impl TestValue {
    fn new(target: u64, values: Vec<u64>) -> Self {
        TestValue {target, values}
    }

    fn is_valid(&self, operators: &Vec<fn(u64, u64) -> u64>) -> bool {
        let mut stack = vec![(self.values[0], 1)];

        while !stack.is_empty(){
            if let Some((total, index)) = stack.pop() {
                if index == self.values.len() {
                    if total == self.target {
                        return true
                    }    
                } else if total <= self.target {
                    stack.extend(operators
                        .iter()
                        .map(|f| (f(total, self.values[index]), index+1))
                    );
                }
            }
        }
        false
    }
}

fn concat(a:u64, b:u64) -> u64 {
    //format!("{}{}", a, b).parse::<u64>().unwrap()
    a * 10_u64.pow(b.ilog10() + 1) + b
}

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

fn run(tests: &Vec<TestValue>, operators: &Vec<fn(u64, u64) -> u64>) -> u64{
    tests.iter()
    .filter_map(|t| {
        match t.is_valid(operators) {
            true => Some(t.target),
            false => None
        }
    })
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
