use std::fs;
use regex::Regex;
use day_17::*;

fn parse(s:&str) -> (Machine, Vec<u32>) {
    let rx = Regex::new(r"\d+").unwrap();
    let nums:Vec<u64> = rx
    .find_iter(&s)
    .map(|d| d.as_str()
        .parse::<u64>()
        .unwrap())
    .collect();

    let machine = Machine::new(nums[0], nums[1], nums[2]);
    let program = nums[3..].iter().map(|&n| n as u32).collect();
    (machine, program)
}
fn main() {
    let input = fs::read_to_string("data.txt").expect("where'd I put my code?");
    let (mut machine, program) = parse(&input);
    println!("{:?} {:?}", machine, program);

    let output = machine.run(program);
    println!("{:?}", output);
}
