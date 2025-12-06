use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    Sum,
    Product
}

impl FromStr for Operator {
    type Err = String;
    fn from_str(op: &str) -> Result<Operator, Self::Err> {

        match op {
            "+" => Ok(Operator::Sum),
            "*" => Ok(Operator::Product),
            _ => Err(format!("Non-cephalopod operator found! {}", op))
        }
   } 
}

fn parse_file_two(raw_data: &str) -> Vec<(Operator, Vec<u64>)>{
    let lines: Vec<Vec<char>> = raw_data.lines().map(|l| l.chars().collect()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let ops = lines[rows-1].iter().filter(|c| !c.is_ascii_whitespace());

    let mut n_gen = (0..cols).map(|col| {
        (0..rows-1)
        .map(|row| lines[row][col])
        .filter(|n| n.is_ascii_digit())
        .collect::<String>()
    });

    ops.map(|op| {
        let mut tmp = [0; 4];
        let s = op.encode_utf8(&mut tmp);
        let operator = s.parse().expect("Bad Operator");

        let nums:Vec<u64> = n_gen.by_ref()
            .take_while(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        (operator, nums)
    })
    .collect()
}

fn parse_file_one(raw_data: &str) -> Vec<(Operator, Vec<u64>)> {
    let lines: Vec<Vec<&str>> = raw_data.lines()
    .map(|line| line.split_whitespace().collect())
    .collect();

    let rows = lines.len();
    let cols = lines[0].len();
    
    let mut n_gen = (0..cols).map(|col| {
        (0..rows-1)
        .map(|row| lines[row][col].parse().unwrap()).collect()
    });
    let ops = &lines[rows-1];

    ops.into_iter().map(|&op| {
        let operator = op.parse().expect("Invalid Operator");
        (operator, n_gen.next().unwrap())
    })
    .collect()
}

fn cephalopod_math(input: &[(Operator,  Vec<u64>)]) -> u64 {
    input.iter()
    .map(|(operator, nums)| match operator {
        Operator::Sum => nums.iter().sum::<u64>(),
        Operator::Product => nums.iter().product(),
    })
    .sum()
}

fn main() {
    let s = read_to_string("data.txt").expect("could not open file");
    
    let answer_one = cephalopod_math(&parse_file_one(&s));
    println!("Part One: {}", answer_one);

    let answer_two = cephalopod_math(&parse_file_two(&s));
    println!("Part Two: {}", answer_two);
}