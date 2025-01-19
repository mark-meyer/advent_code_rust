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

fn _out_n_base_8(o:&Vec<u32>) -> u64 {
    o.iter().fold(0_u64, |a, n| a * 8 + *n as u64 )
}

/*
    The program is loop of 8 instructions. Each loop reduces 
    register 8 by half and outputs a digit. This means for 
    each three bit value in the output register A is three bits
    larger. We can solve by working backward and finding canditates
    for each three bits of register A that output the correct 
    corresponding output. 
*/
fn search(program: &Vec<u32>) -> Option<u64> {
    let mut current = vec![0];
    let mut next = Vec::new();

    for i in (0..program.len()).rev() {
        next.clear();    
        for &partial in &current {
            for d in 0..8 {
                let candidate = (partial << 3) + d ;
                if Machine::new(candidate, 0, 0).run(program) == &program[i..] {
                    next.push(candidate);
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }
    current.into_iter().min()
}

fn main() {
    let input = fs::read_to_string("data.txt").expect("where'd I put my code?");
    let (mut machine, program) = parse(&input);

    let output = machine.run(&program);
    println!("Part one: {:?}", output);

    if let Some(part_two) = search(&program) {
        println!("Part two: {:?}", part_two);
    }

}
