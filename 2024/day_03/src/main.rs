use std::fs;
use regex::Regex;

fn mult_string(s:&str) -> u32 {
    let rx = Regex::new(r"\d+").unwrap();

    rx.find_iter(s)
    .map(|n| n.as_str().parse::<u32>().unwrap())
    .product()
}

fn main() {
    let program = fs::read_to_string("data.txt").expect("Couldn't read file!");
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    // Part One
    let prod:u32 = rx.captures_iter(&program)
    .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
    .sum(); 
    
    println!("Part One: {}", prod);

    // Part Two
    let rx = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut active = true;
    let mut total = 0;

    for found in rx.find_iter(&program) {
        match  found.as_str() {
            "don't()" => active = false,
            "do()" => active = true,
            mul => if active {
                total += mult_string(mul)
            }
        }
    };
    
    println!("Part Two: {}", total);
}
