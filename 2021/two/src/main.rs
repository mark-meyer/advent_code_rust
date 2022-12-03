use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};


static PATH: &'static str = "./data.txt";

#[derive(Debug)]
enum Command {
    Forward(i64),
    Up(i64),
    Down(i64)
}

struct Postion {
    depth:i64,
    horiz:i64
}

use Command::*;

fn read_integers<R: Read>(input: R) -> Vec<Command> {
    let in_data = BufReader::new(input);
    in_data.lines()
    .map(|line| {
        let pair = line.unwrap();
        let mut pair = pair.split_whitespace();
        let pair = (
            pair.next().unwrap(), 
            pair.next().unwrap().parse().expect("Can't parse integer, blame it on Claus")
        );
        match pair {
            ("forward", i) => Forward(i),
            ("up", i)      => Up(i),
            ("down", i)    => Down(i),
            _              => panic!("Malformed command! Elves can't type.")
        }
    }).collect()
    }
    
fn problem_one(commands:&Vec<Command>) -> i64 {
    let mut pos = Postion {depth: 0, horiz: 0};
    for command in commands {
        match command {
            Forward(i) => pos.horiz += i,
            Up(i)      => pos.depth -= i,
            Down(i)    => pos.depth += i,
        }
    }
    return pos.depth * pos.horiz
}

fn problem_two(commands:&Vec<Command>) -> i64 {
    let mut pos = Postion {depth: 0, horiz: 0};
    let mut aim = 0;

    for command in commands {
        match command {
            Forward(i) => {pos.horiz += i; pos.depth += i * aim},
            Up(i)      => aim -= i,
            Down(i)    => aim += i
        }
    }
    return pos.depth * pos.horiz
}

fn main() {
    let path = Path::new(PATH);
    let file = File::open(&path).expect("Error opening file, Merry X-Mas!");

    let commands = read_integers(file);
    let solution_one = problem_one(&commands);
    println!("solution one: {}", solution_one);

    let solution_two = problem_two(&commands);
    println!("solution two: {}", solution_two)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn p_one(){
        let v = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2)
        ];
        assert_eq!(problem_one(&v), 150);
    }

    #[test]
    fn p_two(){
        let v = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2)
        ];
        assert_eq!(problem_two(&v), 900);
    }
}

